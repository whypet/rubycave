use rubycave::{protocol::Packet, rkyv_codec::RkyvCodecError};

pub mod tcp;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rkyv_codec error")]
    RkyvCodec(#[from] RkyvCodecError),
    #[error("rpc packet receive error")]
    Receive,
    #[error("cancelled")]
    Cancelled,
}

pub trait Client {
    async fn send(&mut self, packet: Packet) -> bool;
    async fn receive(&mut self) -> Option<Packet>;
    async fn start(&mut self) -> bool;
    async fn stop(&mut self) -> bool;
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
