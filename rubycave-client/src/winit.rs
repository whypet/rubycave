use std::sync::Arc;

use pollster::FutureExt;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::game::Game;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    game: Option<Game<'static>>,
}

impl App {
    fn render(&self) {
        let game = self.game.as_ref().expect("game not instantiated");
        game.frame();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.window = Some(window.clone());

        self.game = Some(Game::new(window).block_on());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let game = self.game.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                info!("exiting");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                game.get_state().resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => (),
        }
    }
}
