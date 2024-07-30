use rubycave::{protocol::Packet, rkyv_codec::RkyvCodecError};

pub mod tcp;

pub trait RpcClient {
    async fn send(&mut self, packet: Packet);
    async fn receive(&mut self) -> Option<Result<Packet, RkyvCodecError>>;
}

pub enum Client {
    Tcp(tcp::TcpClient),
}
