use std::error::Error;

use rkyv::{Archive, Deserialize, Serialize};
use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Encoder},
};

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

pub struct Codec;

impl Codec {
    const N: usize = 64;
}

impl Encoder<&Packet> for Codec {
    type Error = Box<dyn Error>;

    fn encode(&mut self, item: &Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = rkyv::to_bytes::<Packet, { Self::N }>(item)?;

        dst.extend_from_slice(&bytes);

        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Packet;
    type Error = Box<dyn Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let archived = rkyv::check_archived_root::<Packet>(src)?;
        let deserialized = archived.deserialize(&mut rkyv::Infallible)?;

        Ok(Some(deserialized))
    }
}
