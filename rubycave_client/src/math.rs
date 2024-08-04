use std::ops::{Bound, RangeBounds};

use rubycave::{InfiniteIterator, RangeIterator};

macro_rules! prng_range_impl {
    ($x:ty) => {
        impl<T: RangeBounds<$x>> PrngRange<$x> for T {
            fn prng_bounds(&self) -> ($x, $x) {
                (
                    match self.start_bound() {
                        Bound::Included(x) => *x,
                        Bound::Excluded(x) => *x + 1,
                        Bound::Unbounded => <$x>::MIN,
                    },
                    match self.end_bound() {
                        Bound::Included(x) => *x + 1,
                        Bound::Excluded(x) => *x,
                        Bound::Unbounded => <$x>::MAX,
                    },
                )
            }
        }
    };
}

/// Fast biased Xorshift PRNG
#[derive(Default)]
pub struct FastPrng<T> {
    state: T,
}

trait PrngRange<T> {
    fn prng_bounds(&self) -> (T, T);
}

prng_range_impl!(u64);
prng_range_impl!(i64);
prng_range_impl!(u32);
prng_range_impl!(i32);

impl<T: RangeBounds<u32>> RangeIterator<T> for FastPrng<u32> {
    type Item = u32;

    fn next_in(&mut self, range: T) -> Self::Item {
        let (start, end) = range.prng_bounds();
        (((self.next() as u64) * ((end - start) as u64)) >> 32) as Self::Item + start
    }
}

impl<T: RangeBounds<i32>> RangeIterator<T> for FastPrng<i32> {
    type Item = i32;

    fn next_in(&mut self, range: T) -> Self::Item {
        let (start, end) = range.prng_bounds();
        (((self.next() as i64) * ((end - start) as i64)) >> 32) as Self::Item + start
    }
}

impl<T: RangeBounds<u64>> RangeIterator<T> for FastPrng<u64> {
    type Item = u64;

    fn next_in(&mut self, range: T) -> Self::Item {
        let (start, end) = range.prng_bounds();
        ((self.next() * (end - start)) >> 32) + start
    }
}

impl<T: RangeBounds<i64>> RangeIterator<T> for FastPrng<i64> {
    type Item = i64;

    fn next_in(&mut self, range: T) -> Self::Item {
        let (start, end) = range.prng_bounds();
        ((self.next() * (end - start)) >> 32) + start
    }
}

impl From<FastPrng<u32>> for FastPrng<i32> {
    fn from(value: FastPrng<u32>) -> Self {
        FastPrng::<i32> {
            state: value.state as i32,
        }
    }
}

impl From<FastPrng<i32>> for FastPrng<u32> {
    fn from(value: FastPrng<i32>) -> Self {
        FastPrng::<u32> {
            state: value.state as u32,
        }
    }
}

impl From<FastPrng<u64>> for FastPrng<i64> {
    fn from(value: FastPrng<u64>) -> Self {
        FastPrng::<i64> {
            state: value.state as i64,
        }
    }
}

impl From<FastPrng<i64>> for FastPrng<u64> {
    fn from(value: FastPrng<i64>) -> Self {
        FastPrng::<u64> {
            state: value.state as u64,
        }
    }
}

impl InfiniteIterator for FastPrng<u32> {
    type Item = u32;

    fn next(&mut self) -> Self::Item {
        self.state = xorshift32(self.state);
        self.state
    }
}

impl InfiniteIterator for FastPrng<i32> {
    type Item = i32;

    fn next(&mut self) -> Self::Item {
        self.state = xorshift32(self.state as u32) as i32;
        self.state
    }
}

impl InfiniteIterator for FastPrng<u64> {
    type Item = u64;

    fn next(&mut self) -> Self::Item {
        self.state = xorshift64(self.state);
        self.state
    }
}

impl InfiniteIterator for FastPrng<i64> {
    type Item = i64;

    fn next(&mut self) -> Self::Item {
        self.state = xorshift64(self.state as u64) as i64;
        self.state
    }
}

fn xorshift32(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
