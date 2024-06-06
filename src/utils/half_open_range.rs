use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub enum ClampSign {
    Forwards,  // fix [from_inclusive], clamp by reducing [to_exclusive]
    Backwards, // fix [to_exclusive], clamp by increasing [from_inclusive]
}

impl HalfOpenRange<DateTime<Utc>> {
    pub fn clamp(&self, max_width: chrono::Duration, sign: ClampSign) -> Self {
        match sign {
            ClampSign::Forwards => self.clamp_to(max_width),
            ClampSign::Backwards => self.clamp_from(max_width),
        }
    }

    fn clamp_from(&self, max_width: chrono::Duration) -> Self {
        Self {
            from_inclusive: std::cmp::max(
                self.to_exclusive - max_width,
                self.from_inclusive,
            ),
            ..*self
        }
    }

    fn clamp_to(&self, max_width: chrono::Duration) -> Self {
        Self {
            to_exclusive: std::cmp::min(
                self.from_inclusive + max_width,
                self.to_exclusive,
            ),
            ..*self
        }
    }
}

impl HalfOpenRange<Option<DateTime<Utc>>> {
    pub fn clamp(&self, max_width: chrono::Duration, sign: ClampSign) -> Self {
        match sign {
            ClampSign::Forwards => self.clamp_to(max_width),
            ClampSign::Backwards => self.clamp_from(max_width),
        }
    }

    fn clamp_from(&self, max_width: chrono::Duration) -> Self {
        match (self.from_inclusive, self.to_exclusive) {
            (Some(l), Some(r)) => {
                Self { from_inclusive: Some(std::cmp::max(r - max_width, l)), ..*self }
            }
            _ => *self,
        }
    }

    fn clamp_to(&self, max_width: chrono::Duration) -> Self {
        match (self.from_inclusive, self.to_exclusive) {
            (Some(l), Some(r)) => {
                Self { to_exclusive: Some(std::cmp::min(l + max_width, r)), ..*self }
            }
            _ => *self,
        }
    }
}
