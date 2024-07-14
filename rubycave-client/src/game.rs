use std::sync::Arc;

use winit::window::Window;

use crate::render::Renderer;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub async fn new(window: Arc<Window>) -> Self {
        Self {
            renderer: Renderer::new(window).await,
        }
    }

    pub fn frame(&mut self) {}
}
