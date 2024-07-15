use std::{borrow::Borrow, marker::PhantomData};

use super::{Renderer, State};

pub struct TriangleRenderer<'state, StateRef: Borrow<State<'state>>> {
    #[allow(dead_code)]
    state: StateRef,
    _phantom: PhantomData<&'state ()>,
}

impl<'state, StateRef: Borrow<State<'state>>> Renderer<'state, StateRef>
    for TriangleRenderer<'state, StateRef>
{
    fn new(state: StateRef) -> Self {
        Self {
            state,
            _phantom: PhantomData,
        }
    }

    fn render(&self) {
        todo!()
    }
}
