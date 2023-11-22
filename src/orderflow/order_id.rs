use crate::{utils::component_id::ComponentIdError, ComponentId};
use anyhow::{bail, Result};
use bytes::{Buf, BufMut};
use either::Either;
use netidx::pack::{Pack, PackError};
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderIdGenerator(AtomicU64);

impl Pack for OrderIdGenerator {
    fn encoded_len(&self) -> usize {
        u64::encoded_len(&self.0.load(Ordering::SeqCst))
    }

    fn encode(&self, buf: &mut impl BufMut) -> std::result::Result<(), PackError> {
        u64::encode(&self.0.load(Ordering::SeqCst), buf)
    }

    fn decode(buf: &mut impl Buf) -> std::result::Result<Self, PackError>
    where
        Self: Sized,
    {
        Ok(Self(AtomicU64::new(u64::decode(buf)?)))
    }
}

impl OrderIdGenerator {
    pub fn channel(channel_id: ChannelId) -> Result<Self> {
        Ok(Self(AtomicU64::new(((channel_id.0 as u64) << 40) | 0x1)))
    }

    pub fn component(component_id: ComponentId) -> Self {
        Self(AtomicU64::new(((component_id.0 as u64) << 40) | 0x1))
    }

    pub fn none(&self) -> OrderId {
        OrderId(self.0.load(Ordering::Relaxed) & CHANNEL_ID_MASK)
    }

    pub fn next(&self) -> OrderId {
        OrderId(self.0.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns true if the order_id matches this generator's channel/component mask
    pub fn matches(&self, order_id: OrderId) -> bool {
        self.0.load(Ordering::Relaxed) & CHANNEL_ID_MASK == order_id.0 & CHANNEL_ID_MASK
    }
}

/// System-unique, persistent order identifiers
///
/// 24 bits channel id, or component id (if upper 8 bits are zero)
/// 40 bits sequence number (0 reserved for None)
///
/// Component order ids are considered ephemeral and can be
/// recycled on system restart.
#[derive(
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Pack,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[pack(unwrapped)]
pub struct OrderId(u64);

#[rustfmt::skip] const DISCRIMINANT: u64         = 0xFF00_0000_0000_0000;
#[rustfmt::skip] const CHANNEL_ID_MASK: u64      = 0xFFFF_FF00_0000_0000; // >> 40
#[rustfmt::skip] const COMPONENT_ID_MASK: u64    = 0x00FF_FF00_0000_0000; // >> 40
#[rustfmt::skip] const SEQUENCE_NUMBER_MASK: u64 = 0x0000_00FF_FFFF_FFFF;

impl OrderId {
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        (self.0 & SEQUENCE_NUMBER_MASK) != 0
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        (self.0 & SEQUENCE_NUMBER_MASK) == 0
    }

    #[inline(always)]
    pub fn channel_id(&self) -> Option<ChannelId> {
        if self.0 & DISCRIMINANT == 0 {
            None
        } else {
            Some(ChannelId(((self.0 & CHANNEL_ID_MASK) >> 40) as u32))
        }
    }

    #[inline(always)]
    pub fn component_id(
        &self,
    ) -> Option<std::result::Result<ComponentId, ComponentIdError>> {
        if self.0 & DISCRIMINANT == 0 {
            Some(ComponentId::new(((self.0 & COMPONENT_ID_MASK) >> 40) as u16))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn channel_or_component_id(
        &self,
    ) -> Either<ChannelId, std::result::Result<ComponentId, ComponentIdError>> {
        if let Some(channel_id) = self.channel_id() {
            Either::Left(channel_id)
        } else if let Some(component_id) = self.component_id() {
            Either::Right(component_id)
        } else {
            unreachable!()
        }
    }

    #[inline(always)]
    pub fn sequence_number(&self) -> u64 {
        self.0 & SEQUENCE_NUMBER_MASK
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self(0)
    }
}

impl FromStr for OrderId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str(s)?))
    }
}

impl fmt::Debug for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seq = self.sequence_number();
        match self.channel_or_component_id() {
            Either::Left(cid) => write!(f, "channel:{cid}:{seq}"),
            Either::Right(Ok(cid)) => write!(f, "#{cid}:{seq}"),
            Either::Right(Err(_)) => write!(f, "#<invalid>:{seq}"),
        }
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The id of the channel an order is sent through
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Pack, Serialize, Deserialize,
)]
#[pack(unwrapped)]
pub struct ChannelId(u32);

impl ChannelId {
    pub fn new(id: u32) -> Result<Self> {
        if id & 0xFF0000 == 0 {
            bail!("channel id too small; valid channel ids start from 0x10000")
        }
        Ok(Self(id))
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
