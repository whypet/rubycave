use std::{borrow::Borrow, marker::PhantomData};

use super::{InternalRenderer, Renderer, State};

#[allow(private_bounds)]
pub struct TriangleRenderer<'a, StateRef: Borrow<State<'a>>> {
    state: StateRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, StateRef: Borrow<State<'a>>> InternalRenderer<'a, StateRef>
    for TriangleRenderer<'a, StateRef>
{
    fn new(state: StateRef) -> Self {
        Self {
            state,
            _phantom: PhantomData,
        }
    }
}

impl<'a, StateRef: Borrow<State<'a>>> Renderer for TriangleRenderer<'a, StateRef> {
    fn render(&mut self) {
        todo!()
    }
}
