use crate::{utils::messaging::MaybeRequest, ChannelId};
use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum ChannelAuthorityMessage {
    RequestChannelId(Uuid),
    ChannelId(Uuid, ChannelId),
}

impl MaybeRequest for ChannelAuthorityMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            ChannelAuthorityMessage::RequestChannelId(uuid) => Some(*uuid),
            ChannelAuthorityMessage::ChannelId(..) => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            ChannelAuthorityMessage::RequestChannelId(..) => None,
            ChannelAuthorityMessage::ChannelId(uuid, _) => Some(*uuid),
        }
    }
}

impl ChannelAuthorityMessage {
    pub fn channel_id(&self) -> Result<ChannelId> {
        match self {
            ChannelAuthorityMessage::RequestChannelId(..) => {
                bail!("unexpected response kind")
            }
            ChannelAuthorityMessage::ChannelId(_, channel_id) => Ok(*channel_id),
        }
    }
}
