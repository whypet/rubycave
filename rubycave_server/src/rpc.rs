use std::sync::Arc;

use futures::{SinkExt, Stream, StreamExt};
use rubycave::{
    protocol::{client, server, Packet, PacketValidator},
    rkyv_codec::{futures_stream::RkyvCodec, RkyvCodecError, VarintLength},
};
use tokio_util::codec::Framed;
use tracing::info;

pub mod tcp;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rkyv_codec error")]
    RkyvCodec(#[from] RkyvCodecError),
    #[error("failed to receive data from stream")]
    Receive,
}

pub trait Server<T> {
    async fn accept(&self) -> Option<Framed<T, RkyvCodec<Packet, VarintLength>>>;
}

pub trait StreamClientExt<T> {
    async fn receive(&mut self) -> Result<T, Error>;
}

pub struct Client<T: SinkExt<Packet> + StreamClientExt<Packet> + Unpin> {
    framed: T,
    validator: Arc<PacketValidator>,
}

impl<T: SinkExt<Packet, Error = RkyvCodecError> + StreamClientExt<Packet> + Unpin> Client<T> {
    pub fn new(framed: T, validator: Arc<PacketValidator>) -> Self {
        Self { framed, validator }
    }

    pub async fn receive(&mut self) -> Result<Packet, Error> {
        let packet = self.framed.receive().await?;
        info!("received: {:?}", packet);
        Ok(packet)
    }

    pub async fn send(&mut self, packet: server::Packet) -> Result<(), Error> {
        info!("sending: {:?}", packet);
        Ok(self.framed.send(Packet::Server(packet)).await?)
    }

    pub async fn shake(&mut self) -> Result<bool, Error> {
        self.send(server::Packet::Handshake {
            version: env!("CARGO_PKG_VERSION").to_owned(),
        })
        .await?;

        let packet = self.receive().await?;

        if let Packet::Client(client_packet) = packet {
            if let Err(e) = self.validator.check_client(&client_packet) {
                self.kick(server::KickReason::Packet(e)).await?;
                return Ok(false);
            } else if let client::Packet::Handshake {
                version: _,
                username: _,
            } = client_packet
            {
                return Ok(true);
            }
        }

        self.kick(server::KickReason::Packet(server::PacketError::Handshake))
            .await?;
        Ok(false)
    }

    pub async fn kick(&mut self, reason: server::KickReason) -> Result<(), Error> {
        self.send(server::Packet::Kick { reason }).await
    }
}

impl<T, S: Stream<Item = Result<T, RkyvCodecError>> + Unpin> StreamClientExt<T> for S {
    async fn receive(&mut self) -> Result<T, Error> {
        Ok(self.next().await.ok_or(Error::Receive)??)
    }
}
