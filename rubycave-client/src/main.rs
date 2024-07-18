#![feature(trait_alias)]

use ::winit::event_loop::{ControlFlow, EventLoop};
use color_eyre::{eyre::eyre, Section};
use config::Config;
use tracing::info;

mod config;
mod game;
mod render;
mod resource;
mod winit;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    let config = Config::new(70.0);

    info!("starting");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = winit::App::new(config);

    if let Err(error) = event_loop.run_app(&mut app) {
        Err(eyre!("app failed").with_error(|| error))
    } else {
        Ok(())
    }
}
