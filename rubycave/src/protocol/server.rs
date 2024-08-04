use rkyv::{Archive, Deserialize, Serialize};

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
    Handshake { version: String },
    Kick { reason: KickReason },
}
