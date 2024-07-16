use std::borrow::Borrow;

use crate::resource::ResourceManager;

use super::{triangle::TriangleRenderer, Renderer, State};

pub struct GameRenderer<'state, StateRef: Borrow<State<'state>>> {
    triangle_renderer: TriangleRenderer<'state, StateRef>,
}

impl<'state, StateRef: Borrow<State<'state>>> Renderer<'state, StateRef>
    for GameRenderer<'state, StateRef>
{
    fn new(state_ref: StateRef, resource_man: &'state ResourceManager) -> Self {
        Self {
            triangle_renderer: TriangleRenderer::new(state_ref, resource_man),
        }
    }

    fn render(&self) {
        self.triangle_renderer.render();
    }
}
