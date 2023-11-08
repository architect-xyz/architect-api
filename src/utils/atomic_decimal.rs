use anyhow::Result;
use portable_atomic::{AtomicU128, Ordering as MemOrdering};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A decimal type that can be atomically updated using instructions
/// present in most modern cpus.
#[derive(JsonSchema)]
#[schemars(transparent)]
pub struct AtomicDecimal(#[schemars(with = "Decimal")] AtomicU128);

impl Default for AtomicDecimal {
    fn default() -> Self {
        Self::new(dec!(999999999))
    }
}

impl Serialize for AtomicDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <Decimal as Serialize>::serialize(&self.load(), serializer)
    }
}

impl<'de> Deserialize<'de> for AtomicDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(<Decimal as Deserialize>::deserialize(deserializer)?))
    }
}

impl AtomicDecimal {
    pub fn new(d: Decimal) -> Self {
        Self(AtomicU128::new(u128::from_le_bytes(d.serialize())))
    }

    pub fn load(&self) -> Decimal {
        let buf = self.0.load(MemOrdering::Relaxed);
        Decimal::deserialize(buf.to_le_bytes())
    }

    pub fn store(&self, d: Decimal) {
        let new = u128::from_le_bytes(d.serialize());
        self.0.store(new, MemOrdering::Relaxed);
    }
}

impl PartialEq for AtomicDecimal {
    fn eq(&self, other: &Self) -> bool {
        self.0.load(MemOrdering::Relaxed) == other.0.load(MemOrdering::Relaxed)
    }
}

impl PartialEq<Decimal> for AtomicDecimal {
    fn eq(&self, other: &Decimal) -> bool {
        &self.load() == other
    }
}

impl Eq for AtomicDecimal {}

impl Clone for AtomicDecimal {
    fn clone(&self) -> Self {
        Self(AtomicU128::new(self.0.load(MemOrdering::Relaxed)))
    }
}

impl fmt::Debug for AtomicDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
