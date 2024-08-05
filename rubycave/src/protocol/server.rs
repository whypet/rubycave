use rkyv::{Archive, Deserialize, Serialize};

use crate::world::Chunk;

#[derive(Archive, Deserialize, Serialize, Debug, thiserror::Error)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum PacketError {
    #[error("expected handshake")]
    Handshake,
    #[error("client/server version mismatch")]
    Version,
    #[error("invalid username")]
    Username,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum KickReason {
    Packet(PacketError),
    Operator(String),
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum Packet {
    Handshake {
        version: String,
    },
    Kick {
        reason: KickReason,
    },
    Teleport {
        x: f32,
        y: f32,
        z: f32,
        yaw: f32,
        pitch: f32,
    },
    Chunk(Box<Chunk>),
}
