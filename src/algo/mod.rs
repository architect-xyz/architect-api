#![cfg(feature = "netidx")]

use crate::{
    orderflow::*, utils::messaging::MaybeRequest, AccountId, OrderId, Str, UserId,
};
use anyhow::Result;
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub mod generic_container;
pub mod mm;
pub mod pov;
pub mod smart_order_router;
pub mod twap;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoMessage {
    /// Generally convertible from a specific algo's Order, but not vice versa;
    /// the former conversion is given for AlgoManager to understand the existence
    /// of algo orders.
    AlgoOrder(AlgoOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    ChildAck(ChildAck),
    ChildReject(ChildReject),
    ChildFill(ChildFill),
    ChildOut(ChildOut),
    PreviewAlgo(Uuid, AlgoOrder),
    PreviewAlgoResponse(Uuid, Option<AlgoPreview>),
    GetAlgoOrder(Uuid, OrderId),
    GetAlgoOrderResponse(Uuid, Option<AlgoOrder>),
    GetAlgoStatus(Uuid, Option<OrderId>),
    GetAlgoStatusResponse(Uuid, Arc<Vec<AlgoStatus>>),
    GetAlgoLog(Uuid, OrderId),
    GetAlgoLogResponse(Uuid, Option<AlgoLog>),
}

impl TryInto<OrderflowMessage> for &AlgoMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            AlgoMessage::ChildAck(m) => Ok(OrderflowMessage::Ack((*m).ack)),
            AlgoMessage::ChildReject(m) => {
                Ok(OrderflowMessage::Reject((*m).reject.clone()))
            }
            AlgoMessage::ChildFill(m) => Ok(OrderflowMessage::Fill((*m).fill)),
            AlgoMessage::ChildOut(m) => Ok(OrderflowMessage::Out((*m).out)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AlgoOrder {
    pub order_id: OrderId,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub algo: Str,
}

// CR-someday alee: use something more akin to the validator crate
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoControl {
    pub order_id: OrderId,
    pub command: AlgoControlCommand,
}

impl AlgoControl {
    pub fn start(order_id: OrderId) -> Self {
        Self { order_id, command: AlgoControlCommand::Start }
    }

    pub fn pause(order_id: OrderId) -> Self {
        Self { order_id, command: AlgoControlCommand::Pause }
    }

    pub fn stop(order_id: OrderId) -> Self {
        Self { order_id, command: AlgoControlCommand::Stop }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum AlgoControlCommand {
    Start,
    Pause,
    Stop,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoAck {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoReject {
    pub order_id: OrderId,
    pub reason: ArcStr,
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
    Running,
    Paused,
    Done,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoStatus {
    pub order_id: OrderId,
    pub creation_time: DateTime<Utc>,
    pub status: (DateTime<Utc>, AlgoRunningStatus),
    pub fraction_complete: Option<f64>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct AlgoLog {
    pub order_id: OrderId,
    pub fills: Arc<Vec<Result<Fill, AberrantFill>>>,
    pub rejects: Arc<Vec<Reject>>,
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

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
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
pub struct AlgoPreview {
    pub orders: Arc<Vec<(DateTime<Utc>, Order)>>,
}

// support trivial casing for AlgoContainerMessage: TryInto<AlgoMessage>

impl Into<AlgoOrder> for &AlgoOrder {
    fn into(self) -> AlgoOrder {
        *self
    }
}

impl Into<AlgoPreview> for &AlgoPreview {
    fn into(self) -> AlgoPreview {
        self.clone()
    }
}

impl Into<AlgoStatus> for &AlgoStatus {
    fn into(self) -> AlgoStatus {
        *self
    }
}

impl Into<AlgoLog> for &AlgoLog {
    fn into(self) -> AlgoLog {
        self.clone()
    }
}

impl MaybeRequest for AlgoMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            AlgoMessage::PreviewAlgo(uuid, _)
            | AlgoMessage::GetAlgoStatus(uuid, _)
            | AlgoMessage::GetAlgoLog(uuid, _) => Some(*uuid),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            AlgoMessage::PreviewAlgoResponse(uuid, _)
            | AlgoMessage::GetAlgoStatusResponse(uuid, _)
            | AlgoMessage::GetAlgoLogResponse(uuid, _) => Some(*uuid),
            _ => None,
        }
    }
}
