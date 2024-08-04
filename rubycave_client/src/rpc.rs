use std::io;

use rubycave::{
    protocol::{client, Packet, PacketValidator},
    regex,
    rkyv_codec::RkyvCodecError,
};
use tokio::sync::mpsc;

pub mod tcp;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("regex error")]
    Regex(#[from] regex::Error),
    #[error("rkyv_codec error")]
    RkyvCodec(#[from] RkyvCodecError),
    #[error("mpsc send error")]
    MpscSend(#[from] mpsc::error::SendError<Packet>),
    #[error("mpsc channel closed")]
    MpscClosed(),
    #[error("mpsc try_recv error")]
    MpscTryRecv(#[from] mpsc::error::TryRecvError),
}

pub trait Client {
    fn get_packet_validator(&self) -> &PacketValidator;

    async fn send(&self, packet: client::Packet) -> Result<(), Error>;
    async fn receive(&mut self) -> Result<Packet, Error>;
    async fn poll(&mut self) -> Result<Option<Packet>, Error>;
    async fn start(&mut self) -> bool;
    async fn stop(&mut self) -> bool;

    async fn shake(&mut self, username: &str) -> Result<bool, Error> {
        let packet = self.receive().await?;
        let validator = self.get_packet_validator();

        if let Packet::Server(server_packet) = packet {
            if let Err(e) = validator.check_server(&server_packet) {
                self.disconnect(client::DisconnectReason::Packet(e)).await?;
                return Ok(false);
            }
        }

        self.send(client::Packet::Handshake {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            username: username.to_owned(),
        })
        .await?;

        Ok(true)
    }

    async fn disconnect(&mut self, reason: client::DisconnectReason) -> Result<(), Error> {
        self.send(client::Packet::Disconnect { reason }).await
    }
}
