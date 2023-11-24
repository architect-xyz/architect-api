use crate::ComponentId;
use derive::FromValue;
use netidx_derive::Pack;
use serde::Serialize;

/// Architect components communicate with each other by sending `Envelope`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize)]
pub struct Envelope<M: 'static> {
    pub src: ComponentId,
    pub dst: ComponentId,
    pub msg: M,
}

impl<M> Envelope<M> {
    // external utils/algos should use this fn to construct envelopes
    pub fn to(dst: ComponentId, msg: M) -> Self {
        Self { src: ComponentId::none(), dst, msg }
    }
}
