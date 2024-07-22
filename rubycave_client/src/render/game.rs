use std::{cell::RefCell, rc::Rc};

use crate::{
    config::Config,
    resource::{self, ResourceManager},
};

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
    ) -> Result<Self, resource::Error> {
        Ok(Self {
            world: ChunkRenderer::new(state, config, resource_man, camera)?,
        })
    }
}

impl Renderer for GameRenderer<'_> {
    fn update(&mut self) {
        self.world.update()
    }

    fn render<'p, 'a: 'p>(&'a mut self, frame_view: &wgpu::TextureView) -> wgpu::CommandBuffer {
        self.world.render(frame_view)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.world.resize(width, height)
    }
}
