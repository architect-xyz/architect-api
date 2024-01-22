use super::*;
use crate::{OrderId, Str};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoOrder {
    pub order_id: OrderId,
    pub name: Str,
    pub algo: Str,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
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

#[derive(
    Debug,
    Clone,
    Copy,
    Pack,
    FromValue,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum AlgoRunningStatus {
    NotYetStarted,
    Active,
    Paused,
    Done,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoStatus {
    pub order_id: OrderId,
    pub creation_time: DateTime<Utc>,
    pub status: AlgoRunningStatus,
    pub percent_complete: Option<Decimal>,
    pub fills: Vec<Result<Fill, AberrantFill>>,
    pub rejects: Vec<Reject>,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct ChildAck {
    pub algo_order_id: OrderId,
    pub ack: Ack,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct ChildOut {
    pub algo_order_id: OrderId,
    pub out: Out,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct ChildReject {
    pub algo_order_id: OrderId,
    pub reject: Reject,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct ChildFill {
    pub algo_order_id: OrderId,
    pub fill: Result<Fill, AberrantFill>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoMessage {
    AlgoOrder(AlgoOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    Orderflow(OrderflowMessage),
    ChildAck(ChildAck),
    ChildOut(ChildOut),
    ChildFill(ChildFill),
    ChildReject(ChildReject),
    GetAlgoStatuses(Uuid),
    GetAlgoStatusesResponse(Uuid, Vec<AlgoStatus>),
}

impl Into<AlgoMessage> for &OrderflowMessage {
    fn into(self) -> AlgoMessage {
        AlgoMessage::Orderflow(*self)
    }
}
