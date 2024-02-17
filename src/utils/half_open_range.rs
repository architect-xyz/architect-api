use derive::FromValue;
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize)]
pub struct HalfOpenRange<T: 'static> {
    pub from_inclusive: T,
    pub to_exclusive: T,
}

impl<T> HalfOpenRange<T> {
    pub fn new(from_inclusive: T, to_exclusive: T) -> Self {
        Self { from_inclusive, to_exclusive }
    }
}

impl<T: Copy> HalfOpenRange<T> {
    pub fn infinitesimal(at: T) -> Self {
        Self { from_inclusive: at, to_exclusive: at }
    }
}

impl<T: Ord> HalfOpenRange<T> {
    pub fn contains(&self, x: T) -> bool {
        self.from_inclusive <= x && x < self.to_exclusive
    }
}
