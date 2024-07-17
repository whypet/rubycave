use std::{env, io, rc::Rc};

use crate::{
    config::Config,
    render::{game::GameRenderer, view::Camera, Renderer, State},
    resource::ResourceManager,
};

pub struct Game<'a> {
    state: Rc<State<'a>>,
    renderer: GameRenderer<'a>,
}

impl<'a> Game<'a> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'a>>,
        config: Rc<Config>,
        width: u32,
        height: u32,
    ) -> io::Result<Self> {
        let state = Rc::new(State::new(target, width, height).await);
        let resource_man = Rc::new(ResourceManager::new(env::current_exe()?.parent().unwrap()));
        let camera = Rc::new(Camera::default());

        Ok(Self {
            state: state.clone(),
            renderer: GameRenderer::new(state, config, resource_man, camera),
        })
    }

    pub fn frame(&self) {
        self.renderer.render()
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }
}
