use std::{cell::RefCell, rc::Rc};

use crate::{config::Config, resource::ResourceManager};

use super::{view::Camera, world::ChunkRenderer, Renderer, State};

pub struct GameRenderer<'a> {
    world: ChunkRenderer<'a>,
}

impl<'a> GameRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Self {
        Self {
            world: ChunkRenderer::new(state, config, resource_man, camera),
        }
    }
}

impl Renderer for GameRenderer<'_> {
    fn update(&mut self) {
        self.world.update()
    }

    fn render<'p, 'a: 'p>(&'a mut self, pass: &mut wgpu::RenderPass<'p>) {
        self.world.render(pass)
    }
}
