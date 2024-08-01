use std::{collections::VecDeque, io, sync::Arc};

use futures::SinkExt;
use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, VarintLength},
};
use tokio::{net::TcpStream, select, sync::RwLock, task::JoinHandle};
use tokio_stream::StreamExt;
use tokio_util::{codec::Framed, sync::CancellationToken};
use tracing::info;

use super::{Client, Error};

type TcpFramed = Framed<TcpStream, RkyvCodec<Packet, VarintLength>>;

struct TaskData {
    framed: RwLock<TcpFramed>,
    recv_queue: RwLock<VecDeque<Packet>>,
    send_queue: RwLock<VecDeque<Packet>>,
}

pub struct TcpClient {
    task: Option<JoinHandle<Result<(), Error>>>,
    data: Arc<TaskData>,
    cancel: Option<CancellationToken>,
}

impl TcpClient {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let framed = RwLock::new(Framed::new(
            stream,
            RkyvCodec::<Packet, VarintLength>::default(),
        ));

        let data = Arc::new(TaskData {
            framed,
            recv_queue: Default::default(),
            send_queue: Default::default(),
        });

        Ok(Self {
            task: None,
            data,
            cancel: None,
        })
    }

    async fn process(data: Arc<TaskData>, cancel: CancellationToken) -> Result<(), Error> {
        let mut framed = data.framed.write().await;

        loop {
            let packet = select! {
                p = framed.next() => p.ok_or(Error::Receive)??,
                _ = cancel.cancelled() => break
            };

            info!("received: {:?}", packet);

            {
                let mut recv_queue = data.recv_queue.write().await;
                recv_queue.push_back(packet);
            }
        }

        {
            let mut send_queue = data.send_queue.write().await;

            while let Some(packet) = send_queue.pop_front() {
                framed.send(packet).await?;
            }
        }

        Err(Error::Cancelled)
    }
}

impl Client for TcpClient {
    async fn send(&mut self, packet: Packet) -> bool {
        {
            let mut send_queue = self.data.send_queue.write().await;
            send_queue.push_back(packet);
        }

        if let Some(cancel) = &self.cancel {
            cancel.cancel();
            true
        } else {
            false
        }
    }

    async fn receive(&mut self) -> Option<Packet> {
        {
            let mut recv_queue = self.data.recv_queue.write().await;
            recv_queue.pop_front()
        }
    }

    async fn start(&mut self) -> bool {
        if let Some(task) = self.task.take() {
            if let Some(cancel) = &mut self.cancel {
                if !cancel.is_cancelled() && !task.is_finished() {
                    return false;
                }
            }

            let _ = task.await;
        }

        let data = self.data.clone();
        let cancel = CancellationToken::new();
        let task_cancel = cancel.clone();

        self.cancel = Some(cancel);

        self.task = Some(tokio::spawn(async move {
            Self::process(data, task_cancel).await
        }));

        true
    }

    async fn stop(&mut self) -> bool {
        if let Some(task) = &self.task {
            task.abort();
            true
        } else {
            false
        }
    }
}

impl Drop for TcpClient {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}
