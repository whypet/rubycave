use std::{cell::RefCell, rc::Rc};

use crate::{config::Config, resource::ResourceManager};

use super::{test::TriangleRenderer, view::Camera, world::WorldRenderer, Renderer, State};

pub struct GameRenderer<'a> {
    triangle: TriangleRenderer<'a>,
    world: WorldRenderer<'a>,
}

impl<'a> GameRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Self {
        Self {
            triangle: TriangleRenderer::new(
                state.clone(),
                config.clone(),
                resource_man,
                camera.clone(),
            ),
            world: WorldRenderer::new(state, config, camera),
        }
    }
}

impl Renderer for GameRenderer<'_> {
    fn render(&mut self) {
        self.triangle.render();
        // self.world.render();
    }
}
