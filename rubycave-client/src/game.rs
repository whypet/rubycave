use ouroboros::self_referencing;

use crate::{
    render::{game::GameRenderer, view::Camera, Renderer, State},
    resource::ResourceManager,
};

#[self_referencing]
struct InnerGame<'state> {
    state: State<'state>,
    resource_man: ResourceManager,
    camera: Camera,
    #[borrows(state, resource_man, camera)]
    #[not_covariant]
    renderer: GameRenderer<'this, &'this State<'this>>,
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
                Camera::default(),
                |s, r, c| GameRenderer::new(s, r, c),
            ),
        }
    }

    pub fn frame(&self) {
        self.inner.with_renderer(|renderer| {
            renderer.render();
        });
    }

    pub fn get_state(&self) -> &State {
        self.inner.with_state(|s| s)
    }
}
