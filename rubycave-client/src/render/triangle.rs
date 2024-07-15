use std::marker::PhantomData;

use super::{InternalRenderer, Renderer, State};

#[allow(private_bounds)]
pub struct TriangleRenderer<'a, StateRef: AsRef<State<'a>>> {
    state: StateRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, StateRef: AsRef<State<'a>>> InternalRenderer<'a, StateRef>
    for TriangleRenderer<'a, StateRef>
{
    fn new(state: StateRef) -> Self {
        Self {
            state,
            _phantom: PhantomData,
        }
    }
}

impl<'a, StateRef: AsRef<State<'a>>> Renderer for TriangleRenderer<'a, StateRef> {
    fn render(&mut self) {
        todo!()
    }
}
