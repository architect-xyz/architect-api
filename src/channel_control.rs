#![cfg(feature = "netidx")]

use crate::{MessageTopic, UserId};
use derive::FromValue;
use enumflags2::BitFlags;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum ChannelControlMessage {
    ChannelSubscribe(UserId, u32, BitFlags<MessageTopic>),
}
