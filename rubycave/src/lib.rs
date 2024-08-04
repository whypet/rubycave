use std::{
    ops::RangeBounds,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub use glam;
pub use regex;
pub use rkyv_codec;

pub mod protocol;
pub mod world;

pub trait InfiniteIterator {
    type Item;

    fn next(&mut self) -> Self::Item;
}

pub trait RangeIterator<T: RangeBounds<Self::Item>> {
    type Item;

    fn next_in(&mut self, range: T) -> Self::Item;
}

impl<Item> Iterator for dyn InfiniteIterator<Item = Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

pub fn epoch() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}
