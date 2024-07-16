use std::borrow::Borrow;

use crate::resource::ResourceManager;

use super::{triangle::TriangleRenderer, view::Camera, Renderer, State};

pub struct GameRenderer<'a, StateRef: Borrow<State<'a>>> {
    triangle_renderer: TriangleRenderer<'a, StateRef>,
}

impl<'a, StateRef: Borrow<State<'a>>> GameRenderer<'a, StateRef> {
    pub fn new(state_ref: StateRef, resource_man: &'a ResourceManager, camera: &'a Camera) -> Self {
        Self {
            triangle_renderer: TriangleRenderer::new(state_ref, resource_man, camera),
        }
    }
}

impl<'a, StateRef: Borrow<State<'a>>> Renderer<'a, StateRef> for GameRenderer<'a, StateRef> {
    fn render(&self) {
        self.triangle_renderer.render();
    }
}
