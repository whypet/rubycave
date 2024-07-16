use ouroboros::self_referencing;

use crate::{
    render::{game::GameRenderer, Renderer, State},
    resource::ResourceManager,
};

#[self_referencing]
struct InnerGame<'state> {
    state: State<'state>,
    resource_man: ResourceManager,
    #[borrows(state, resource_man)]
    #[not_covariant]
    inner_renderer: GameRenderer<'this, &'this State<'this>>,
}

pub struct Game<'a> {
    inner: InnerGame<'a>,
}

impl<'a> Game<'a> {
    pub async fn new(target: impl Into<wgpu::SurfaceTarget<'a>>) -> Self {
        Self {
            inner: InnerGame::new(
                State::new(target).await,
                ResourceManager::new(
                    std::env::current_exe()
                        .expect("failed to get current executable path")
                        .parent()
                        .expect("failed to get current executable parent directory"),
                ),
                |s, r| GameRenderer::new(s, r),
            ),
        }
    }

    pub fn frame(&self) {
        self.inner.with_inner_renderer(|renderer| {
            renderer.render();
        });
    }

    pub fn get_state(&self) -> &State {
        self.inner.with_state(|s| s)
    }
}
