use std::{
    ops::RangeBounds,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub use glam;
pub use regex;
pub use rkyv_codec;
pub use tokio_util;

pub mod protocol;
pub mod world;

pub const TICK_RATE: u32 = 60;
pub const KEEP_ALIVE_INTERVAL: u32 = 5000;

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
