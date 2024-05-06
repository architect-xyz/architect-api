use crate::{ComponentId, UserId};
use anyhow::Result;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Pack, FromValue, Serialize, Deserialize,
)]
pub enum Address {
    Component(ComponentId),
    Channel(UserId, u32),
}

impl From<ComponentId> for Address {
    #[inline(always)]
    fn from(id: ComponentId) -> Self {
        Self::Component(id)
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Component(id) => write!(f, "#{}", id),
            Address::Channel(user_id, channel) => write!(f, "{}:{}", user_id, channel),
        }
    }
}

impl Address {
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        match self {
            Address::Component(id) => id.is_loopback(),
            Address::Channel(..) => false,
        }
    }

    #[inline(always)]
    pub fn component<T>(id: T) -> Result<Self>
    where
        T: TryInto<ComponentId>,
        <T as TryInto<ComponentId>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Ok(Self::Component(id.try_into()?))
    }
}

/// Architect components communicate with each other by sending `Envelope`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct Envelope<M: 'static> {
    pub src: Address,
    pub dst: Address,
    pub stamp: Stamp,
    pub msg: M,
}

impl<M> Envelope<M> {
    pub fn system_control(msg: M) -> Self {
        Self {
            src: Address::Component(ComponentId::none()),
            dst: Address::Component(ComponentId::none()),
            stamp: Stamp::default(),
            msg,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct Stamp {
    pub user_id: Option<UserId>,
    pub sequence: Option<Sequence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub enum Sequence {
    Local(u64),
    Remote { core_id: Uuid, last_seqno: Option<u64>, seqno: u64 },
}
