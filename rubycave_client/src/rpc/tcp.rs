use std::sync::Arc;

use futures::SinkExt;
use rubycave::{
    protocol::{client, Packet, PacketValidator},
    rkyv_codec::{futures_stream::RkyvCodec, VarintLength},
};
use tokio::{
    net::TcpStream,
    select,
    sync::{
        mpsc::{self, error::TryRecvError},
        RwLock,
    },
    task::JoinHandle,
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::info;

use super::{Client, Error};

type TcpFramed = Framed<TcpStream, RkyvCodec<Packet, VarintLength>>;

struct TaskData {
    framed: RwLock<TcpFramed>,
    recv: mpsc::Sender<Packet>,
    send: RwLock<mpsc::Receiver<Packet>>,
}

pub struct TcpClient {
    validator: PacketValidator,
    data: Arc<TaskData>,
    task: Option<JoinHandle<Result<(), Error>>>,
    recv: mpsc::Receiver<Packet>,
    send: mpsc::Sender<Packet>,
}

impl TcpClient {
    pub async fn new(addr: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let framed = RwLock::new(Framed::new(
            stream,
            RkyvCodec::<Packet, VarintLength>::default(),
        ));

        let validator = PacketValidator::new(env!("CARGO_PKG_VERSION"))?;

        let send = mpsc::channel(32);
        let recv = mpsc::channel(32);

        let data = Arc::new(TaskData {
            framed,
            recv: recv.0,
            send: RwLock::new(send.1),
        });

        Ok(Self {
            validator,
            data,
            task: None,
            recv: recv.1,
            send: send.0,
        })
    }

    async fn client_task(data: Arc<TaskData>) -> Result<(), Error> {
        let mut framed = data.framed.write().await;
        let recv = &data.recv;
        let mut send = data.send.write().await;

        loop {
            select! {
                received = framed.next() => if let Some(p) = received {
                    let p = p?;
                    info!("received: {:?}", p);
                    recv.send(p).await?;
                },
                sent = send.recv() => if let Some(p) = sent {
                    info!("sending: {:?}", p);
                    framed.send(p).await?;
                },
            };
        }
    }
}

impl Client for TcpClient {
    fn get_packet_validator(&self) -> &PacketValidator {
        &self.validator
    }

    async fn send(&self, packet: client::Packet) -> Result<(), Error> {
        Ok(self.send.send(Packet::Client(packet)).await?)
    }

    async fn receive(&mut self) -> Result<Packet, Error> {
        self.recv.recv().await.ok_or(Error::MpscClosed())
    }

    async fn poll(&mut self) -> Result<Option<Packet>, Error> {
        match self.recv.try_recv() {
            Err(TryRecvError::Empty) => Ok(None),
            res => Ok(Some(res?)),
        }
    }

    async fn start(&mut self) -> bool {
        if let Some(task) = self.task.take() {
            let _ = task.await;
        }

        let data = self.data.clone();
        self.task = Some(tokio::spawn(async move { Self::client_task(data).await }));

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
