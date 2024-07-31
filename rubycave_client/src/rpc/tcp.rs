use std::io;

use futures::{SinkExt, StreamExt};
use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, RkyvCodecError, VarintLength},
};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use super::Client;

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        Ok(Self { stream })
    }
}

impl Client for TcpClient {
    async fn send(&mut self, packet: Packet) {
        let mut transport = FramedWrite::new(
            &mut self.stream,
            RkyvCodec::<Packet, VarintLength>::default(),
        );
        transport.send(packet).await.unwrap();
    }

    async fn receive(&mut self) -> Option<Result<Packet, RkyvCodecError>> {
        let mut transport = FramedRead::new(
            &mut self.stream,
            RkyvCodec::<Packet, VarintLength>::default(),
        );

        transport.next().await
    }
}
