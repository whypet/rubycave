use std::io;

use tokio::net::TcpListener;

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
            let (stream, addr) = self.listener.accept().await?;
            tokio::spawn(async move {
                // todo
            });
        }
    }
}
