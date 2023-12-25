use crate::ComponentId;
use derive::FromValue;
use netidx_derive::Pack;
use serde::Serialize;
use uuid::Uuid;

/// Architect components communicate with each other by sending `Envelope`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct Envelope<M: 'static> {
    pub src: ComponentId,
    pub dst: ComponentId,
    pub stamp: Stamp,
    pub msg: M,
}

impl<M> Envelope<M> {
    // external utils/algos should use this fn to construct envelopes
    pub fn to(dst: ComponentId, msg: M) -> Self {
        Self { src: ComponentId::none(), dst, stamp: Stamp::Unstamped, msg }
    }

    pub fn system_control(msg: M) -> Self {
        Self {
            src: ComponentId::none(),
            dst: ComponentId::none(),
            stamp: Stamp::Unstamped,
            msg,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub enum Stamp {
    Unstamped,
    Local(u64),
    Remote(RemoteStamp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct RemoteStamp {
    pub core_id: Uuid,
    pub last_seqno: Option<u64>,
    pub seqno: u64,
}
