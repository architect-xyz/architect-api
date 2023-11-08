use anyhow::Result;
use enumflags2::{BitFlag, BitFlags, _internal::RawBitFlags};
use portable_atomic::Ordering as MemOrdering;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, marker::PhantomData, sync::atomic::AtomicU64};

/// A wrapper around repr(u64) bitflags types that can be atomically updated
pub struct AtomicBitFlags<T: BitFlag> {
    bits: AtomicU64,
    ph: PhantomData<T>,
}

impl<T> AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn new(t: BitFlags<T>) -> Self {
        Self { bits: AtomicU64::new(t.bits()), ph: PhantomData }
    }

    pub fn load(&self) -> BitFlags<T> {
        BitFlags::from_bits_truncate(self.bits.load(MemOrdering::Relaxed))
    }

    pub fn store(&self, t: BitFlags<T>) {
        self.bits.store(t.bits(), MemOrdering::Relaxed)
    }
}

impl<T> Serialize for AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <BitFlags<T> as Serialize>::serialize(&self.load(), serializer)
    }
}

impl<'de, T> Deserialize<'de> for AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(<BitFlags<T> as Deserialize>::deserialize(deserializer)?))
    }
}

impl<T> Clone for AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn clone(&self) -> Self {
        Self {
            bits: AtomicU64::new(self.bits.load(MemOrdering::Relaxed)),
            ph: PhantomData,
        }
    }
}

impl<T> PartialEq for AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn eq(&self, other: &Self) -> bool {
        self.bits.load(MemOrdering::Relaxed) == other.bits.load(MemOrdering::Relaxed)
    }
}

impl<T> PartialEq<BitFlags<T>> for AtomicBitFlags<T>
where
    T: BitFlag + RawBitFlags<Numeric = u64>,
{
    fn eq(&self, other: &BitFlags<T>) -> bool {
        &self.load() == other
    }
}

impl<T> Eq for AtomicBitFlags<T> where T: BitFlag + RawBitFlags<Numeric = u64> {}

impl<T> fmt::Debug for AtomicBitFlags<T>
where
    T: fmt::Debug + BitFlag + RawBitFlags<Numeric = u64>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.load())
    }
}
