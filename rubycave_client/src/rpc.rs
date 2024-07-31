use rubycave::{protocol::Packet, rkyv_codec::RkyvCodecError};

pub mod tcp;

pub trait Client {
    async fn send(&mut self, packet: Packet);
    async fn receive(&mut self) -> Option<Result<Packet, RkyvCodecError>>;
}
/*
pub enum RpcClient {
    Tcp(tcp::TcpClient),
}

impl Client for RpcClient {
    async fn send(&mut self, packet: Packet) {
        match self {
            RpcClient::Tcp(c) => c.send(packet).await,
        }
    }

    async fn receive(&mut self) -> Option<Result<Packet, RkyvCodecError>> {
        match self {
            RpcClient::Tcp(c) => c.receive().await,
        }
    }
}
*/
