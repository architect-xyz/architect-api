#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct Amount<T: 'static, U: 'static> {
    amount: T,
    unit: U,
}

impl<T: 'static, U: 'static> Amount<T, U> {
    pub fn new(amount: T, unit: U) -> Self {
        Self { amount, unit }
    }

    pub fn as_scalar(&self) -> &T {
        &self.amount
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }
}

impl<T: Ord + 'static, U: Eq + 'static> PartialOrd for Amount<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.unit == other.unit {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}
