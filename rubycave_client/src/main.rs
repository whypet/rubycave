use color_eyre::{
    eyre::{self, eyre},
    Section,
};
use config::Config;
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

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    let config = Config {
        fov: 70.0,
        sensitivity: 0.5,
    };

    info!("starting");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = window::winit::App::new(config)?;

    if let Err(error) = event_loop.run_app(&mut app) {
        Err(eyre!("app failed").with_error(|| error))
    } else {
        Ok(())
    }
}
