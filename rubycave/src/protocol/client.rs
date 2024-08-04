use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, thiserror::Error)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum PacketError {
    #[error("expected handshake")]
    Handshake,
    #[error("client/server version mismatch")]
    Version,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum DisconnectReason {
    Packet(PacketError),
    Player,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum Packet {
    Handshake { version: String, username: String },
    Disconnect { reason: DisconnectReason },
    KeepAlive { epoch: u64 },
}
