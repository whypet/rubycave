use std::io;

use futures::{SinkExt, StreamExt};
use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, RkyvCodecError, VarintLength},
};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use super::Client;

pub struct TcpClient {
    framed: Framed<TcpStream, RkyvCodec<Packet, VarintLength>>,
}

impl TcpClient {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let framed = Framed::new(stream, RkyvCodec::<Packet, VarintLength>::default());

        Ok(Self { framed })
    }
}

impl Client for TcpClient {
    async fn send(&mut self, packet: Packet) {
        self.framed.send(packet).await.unwrap();
    }

    async fn receive(&mut self) -> Option<Result<Packet, RkyvCodecError>> {
        self.framed.next().await
    }
}
