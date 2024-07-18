use std::{cell::RefCell, rc::Rc};

use crate::{config::Config, resource::ResourceManager};

use super::{triangle::TriangleRenderer, view::Camera, Renderer, State};

pub struct GameRenderer<'a> {
    triangle_renderer: TriangleRenderer<'a>,
}

impl<'a> GameRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Self {
        Self {
            triangle_renderer: TriangleRenderer::new(state, config, resource_man, camera),
        }
    }
}

impl Renderer for GameRenderer<'_> {
    fn render(&self) {
        self.triangle_renderer.render();
    }
}
