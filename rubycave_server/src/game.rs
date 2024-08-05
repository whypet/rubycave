use std::{io, sync::Arc};

use rubycave::{
    protocol::{Packet, PacketValidator},
    regex,
    rkyv_codec::{futures_stream::RkyvCodec, RkyvCodecError, VarintLength},
    tokio_util::codec::Framed,
};
use tokio::net::TcpStream;
use tracing::info;

use crate::rpc::{self, tcp::TcpServer, Client, Server};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("regex error")]
    Regex(#[from] regex::Error),
    #[error("rpc error")]
    Rpc(#[from] rpc::Error),
    #[error("rkyv_codec error")]
    RkyvCodec(#[from] RkyvCodecError),
}

pub struct Game {
    server: TcpServer,
    validator: Arc<PacketValidator>,
}

impl Game {
    pub async fn new() -> Result<Self, Error> {
        let server = TcpServer::new("0.0.0.0:1616").await?;
        let validator = Arc::new(PacketValidator::new(env!("CARGO_PKG_VERSION"))?);

        Ok(Self { server, validator })
    }

    pub async fn run(&self) -> Option<()> {
        loop {
            let framed = self.server.accept().await?;
            let client = Client::new(framed, self.validator.clone());

            tokio::spawn(async move { Self::client_task(client).await });
        }
    }

    async fn client_task(
        mut client: Client<Framed<TcpStream, RkyvCodec<Packet, VarintLength>>>,
    ) -> Result<(), Error> {
        info!("new client");

        if !client.shake().await? {
            return Ok(());
        }

        loop {
            let _ = client.receive().await?;
        }
    }
}
