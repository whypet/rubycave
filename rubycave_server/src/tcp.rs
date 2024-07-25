use std::io;

use rubycave::protocol::Codec;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;

        Ok(Self { listener })
    }

    pub async fn run(&self) -> io::Result<()> {
        loop {
            let (stream, _) = self.listener.accept().await?;

            tokio::spawn(async move {
                let mut transport = Framed::new(stream, Codec);

                while let Some(packet) = transport.next().await {
                    // todo
                }
            });
        }
    }
}
