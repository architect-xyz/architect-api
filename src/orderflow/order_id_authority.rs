use crate::utils::messaging::MaybeRequest;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum OrderAuthorityMessage {
    RequestAllocation(Uuid, u64),
    Allocation(Uuid, Option<OrderIdAllocation>),
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct OrderIdAllocation {
    pub allocation_min: u64,
    pub allocation_max: u64,
}

impl MaybeRequest for OrderAuthorityMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            OrderAuthorityMessage::RequestAllocation(uuid, _) => Some(*uuid),
            OrderAuthorityMessage::Allocation(..) => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            OrderAuthorityMessage::RequestAllocation(..) => None,
            OrderAuthorityMessage::Allocation(uuid, _) => Some(*uuid),
        }
    }
}
