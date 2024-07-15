use ouroboros::self_referencing;
use wgpu::SurfaceTarget;

use crate::render::{game::GameRenderer, Renderer, State};

#[self_referencing]
struct GameRendererBox<'state> {
    state: State<'state>,
    #[borrows(state)]
    #[not_covariant]
    inner_renderer: GameRenderer<'this, &'this State<'this>>,
}

pub struct Game<'a> {
    renderer_box: GameRendererBox<'a>,
}

impl<'a> Game<'a> {
    pub async fn new(target: impl Into<SurfaceTarget<'a>>) -> Self {
        Self {
            renderer_box: GameRendererBox::new(State::new(target).await, |s| GameRenderer::new(s)),
        }
    }

    pub fn frame(&'a self) {
        self.renderer_box.with_inner_renderer(|renderer| {
            renderer.render();
        });
    }
}
