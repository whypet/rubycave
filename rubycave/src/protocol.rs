use rkyv::{Archive, Deserialize, Serialize};

pub mod client;
pub mod server;

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum Packet {
    Handshake { version: String },
    Client(client::Packet),
    Server(server::Packet),
}
