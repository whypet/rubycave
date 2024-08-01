use std::{collections::VecDeque, io, sync::Arc};

use futures::{channel::oneshot::Cancellation, SinkExt};
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
    token: RwLock<CancellationToken>,
}

pub struct TcpClient {
    task: Option<JoinHandle<Result<(), Error>>>,
    data: Arc<TaskData>,
}

impl TaskData {
    pub async fn token(&self) -> CancellationToken {
        self.token.read().await.clone()
    }

    pub async fn cancel(&self) {
        let token = self.token.read().await;
        token.cancel();
    }

    pub async fn reset(&self) {
        let mut token = self.token.write().await;
        *token = CancellationToken::new();
    }

    pub async fn is_cancelled(&self) -> bool {
        self.token.read().await.is_cancelled()
    }
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
            token: Default::default(),
        });

        Ok(Self { task: None, data })
    }

    async fn process(data: Arc<TaskData>) -> Result<(), Error> {
        let mut framed = data.framed.write().await;

        loop {
            let token = data.token().await;

            loop {
                let packet = select! {
                    p = framed.next() => p.ok_or(Error::Receive)??,
                    _ = token.cancelled() => break
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

            data.reset().await;
        }
    }
}

impl Client for TcpClient {
    async fn send(&mut self, packet: Packet) -> bool {
        {
            let mut send_queue = self.data.send_queue.write().await;
            send_queue.push_back(packet);
        }

        self.data.cancel().await;

        true
    }

    async fn receive(&mut self) -> Option<Packet> {
        let mut recv_queue = self.data.recv_queue.write().await;
        recv_queue.pop_front()
    }

    async fn start(&mut self) -> bool {
        if let Some(task) = self.task.take() {
            if !self.data.is_cancelled().await && !task.is_finished() {
                return false;
            }

            let _ = task.await;
        }

        let data = self.data.clone();
        self.task = Some(tokio::spawn(async move { Self::process(data).await }));

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
