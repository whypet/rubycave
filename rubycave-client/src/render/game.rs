use std::borrow::Borrow;

use super::{triangle::TriangleRenderer, Renderer, State};

pub struct GameRenderer<'state, StateRef: Borrow<State<'state>>> {
    triangle_renderer: TriangleRenderer<'state, StateRef>,
}

impl<'state, StateRef: Borrow<State<'state>>> Renderer<'state, StateRef>
    for GameRenderer<'state, StateRef>
{
    fn new(state: StateRef) -> Self {
        Self {
            triangle_renderer: TriangleRenderer::new(state),
        }
    }

    fn render(&self) {
        self.triangle_renderer.render();
    }
}
