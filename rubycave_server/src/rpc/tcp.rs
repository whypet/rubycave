use std::io;

use futures::{SinkExt, StreamExt};
use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, RkyvCodecError, VarintLength},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

use super::RpcServer;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rkyv_codec error")]
    RkyvCodec(#[from] RkyvCodecError),
    #[error("failed to receive data from stream")]
    Receive,
}

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;

        Ok(Self { listener })
    }

    pub async fn run(&self) -> Option<()> {
        loop {
            let Some(mut framed) = self.accept().await else {
                return None;
            };

            tokio::spawn(async move {
                framed
                    .send(Packet::Handshake {
                        version: env!("CARGO_PKG_VERSION").to_owned(),
                    })
                    .await?;

                while let Ok(Ok(packet)) = framed.next().await.ok_or(Error::Receive) {
                    println!("Received: {:?}", packet);
                }

                Result::<(), Error>::Ok(())
            });
        }
    }
}

impl RpcServer<TcpStream> for TcpServer {
    async fn accept(&self) -> Option<Framed<TcpStream, RkyvCodec<Packet, VarintLength>>> {
        let Ok((stream, _)) = self.listener.accept().await else {
            return None;
        };

        Some(Framed::new(
            stream,
            RkyvCodec::<Packet, VarintLength>::default(),
        ))
    }
}
