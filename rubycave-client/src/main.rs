use ::winit::event_loop::{ControlFlow, EventLoop};
use color_eyre::{eyre::eyre, Section};
use tracing::info;
use winit::App;

mod game;
mod render;
mod resource;
mod winit;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    info!("starting");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    if let Err(error) = event_loop.run_app(&mut app) {
        Err(eyre!("app failed").with_error(|| error))
    } else {
        Ok(())
    }
}
