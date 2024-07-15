use wgpu::SurfaceTarget;

use crate::render::{GameRenderer, Renderer};

pub struct Game<'window> {
    renderer: GameRenderer<'window>,
}

impl<'window> Game<'window> {
    pub async fn new(target: impl Into<SurfaceTarget<'window>>) -> Self {
        Self {
            renderer: GameRenderer::new(target).await,
        }
    }

    pub fn frame(&mut self) {
        self.renderer.render();
    }
}
