use rubycave::{
    protocol::Packet,
    rkyv_codec::{futures_stream::RkyvCodec, VarintLength},
};
use tokio_util::codec::Framed;

pub mod tcp;

pub trait RpcServer<T> {
    async fn accept(&self) -> Option<Framed<T, RkyvCodec<Packet, VarintLength>>>;
}
