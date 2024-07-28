use std::time::{self, SystemTime};

use color_eyre::{eyre::eyre, Section};
use config::Config;
use rpc::{tcp::TcpClient, RpcClient};
use rubycave::protocol::{self, Packet};
use tracing::info;
use winit::event_loop::{ControlFlow, EventLoop};

mod config;
mod entity;
mod game;
mod render;
mod resource;
mod rpc;
mod window;

pub const TEXTURE_DIR: &str = env!("TEXTURE_DIR");
pub const SHADER_DIR: &str = env!("SHADER_DIR");

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // test tcp client
    rt.block_on(async {
        let mut client = TcpClient::new("127.0.0.1:1616").await.unwrap();

        client
            .send(Packet::Client(protocol::client::Packet::KeepAlive {
                epoch: SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }))
            .await;

        while let Some(Ok(packet)) = client.receive().await {
            println!("Received: {:?}", packet);
        }
    });

    let config = Config {
        fov: 70.0,
        sensitivity: 0.5,
    };

    info!("starting");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = window::winit::App::new(config);

    if let Err(error) = event_loop.run_app(&mut app) {
        Err(eyre!("app failed").with_error(|| error))
    } else {
        Ok(())
    }
}
