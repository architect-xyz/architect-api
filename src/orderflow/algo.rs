use super::*;
use crate::{ComponentId, OrderId, Str};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoOrder {
    pub order_id: OrderId,
    pub name: Str,
    pub algo_component: ComponentId,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoControlCommand {
    Start,
    Pause,
    End,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoControl {
    pub order_id: OrderId,
    pub command: AlgoControlCommand,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoAck {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoReject {
    pub order_id: OrderId,
    pub reason: Str,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoRunningStatus {
    NotYetStarted,
    Active,
    Paused,
    Done,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoStatus {
    pub status: AlgoRunningStatus,
    pub percent_complete: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoMessage {
    AlgoOrder(AlgoOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    ChildAck(Ack),
    ChildOut(Out),
    ChildFill(Result<Fill, AberrantFill>),
    ChildReject(Reject),
}

impl TryInto<AlgoMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<AlgoMessage, ()> {
        match self {
            OrderflowMessage::Order(_) => Err(()),
            OrderflowMessage::Cancel(_) => Err(()),
            OrderflowMessage::Reject(r) => Ok(AlgoMessage::ChildReject(*r)),
            OrderflowMessage::Ack(a) => Ok(AlgoMessage::ChildAck(*a)),
            OrderflowMessage::Fill(f) => Ok(AlgoMessage::ChildFill(*f)),
            OrderflowMessage::Out(o) => Ok(AlgoMessage::ChildOut(*o)),
        }
    }
}
