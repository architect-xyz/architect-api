use crate::{ComponentId, MessageTopic, TypedMessage, UserId};
use anyhow::Result;
use derive::FromValue;
use enumflags2::BitFlags;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Pack, FromValue, Serialize, Deserialize,
)]
pub enum Address {
    Component(ComponentId),
    Channel(UserId, u32),
    /// For cases like external orderflow where the message doesn't ultimately route
    /// to any particular client; in this case the message can only be picked up via
    /// a channel subscription.  This is different from Component(#none) which
    /// actually goes nowhere and is reserved for SystemControl messages.
    ChannelSubscriptionOnly,
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
            Address::ChannelSubscriptionOnly => write!(f, "~"),
        }
    }
}

impl Address {
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        match self {
            Address::Component(id) => id.is_loopback(),
            Address::Channel(..) => false,
            Address::ChannelSubscriptionOnly => false,
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
            stamp: Stamp::new(Default::default()),
            msg,
        }
    }
}

impl Envelope<TypedMessage> {
    pub fn topics(&self) -> BitFlags<MessageTopic> {
        self.msg.topics() | self.stamp.additional_topics
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct Stamp {
    pub user_id: Option<UserId>,
    pub sequence: Option<Sequence>,
    pub additional_topics: BitFlags<MessageTopic>,
}

impl Stamp {
    pub fn new(additional_topics: BitFlags<MessageTopic>) -> Self {
        Self { user_id: None, sequence: None, additional_topics }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub enum Sequence {
    Local(u64),
    Remote { core_id: Uuid, last_seqno: Option<u64>, seqno: u64 },
}
