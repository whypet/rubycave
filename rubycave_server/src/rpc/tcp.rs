use std::io;

use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, VarintLength},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

use super::Server;

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;

        Ok(Self { listener })
    }
}

impl Server<TcpStream> for TcpServer {
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
