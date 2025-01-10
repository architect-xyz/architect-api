#![cfg(feature = "netidx")]

use crate::{
    orderflow::*, symbology::MarketId, utils::messaging::MaybeRequest, AccountId,
    OrderId, UserId,
};
use anyhow::Result;
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strum_macros::EnumIter;
use uuid::Uuid;

pub mod chaser;
pub mod generic_container;
pub mod mm;
pub mod pov;
pub mod quote_one_side;
pub mod smart_order_router;
pub mod spreader;
pub mod take_and_chase;
pub mod twap;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub enum AlgoMessage {
    /// Generally convertible from a specific algo's Order, but not vice versa;
    /// the former conversion is given for AlgoManager to understand the existence
    /// of algo orders.
    AlgoOrder(AlgoOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    AlgoOut(AlgoOut),
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
    AlgoModify(Uuid, AlgoModify),
    AlgoModifyAccept(Uuid, AlgoModify),
    AlgoModifyReject(Uuid),
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

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AlgoOrder {
    pub order_id: OrderId,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub algo: AlgoKind,
    pub parent_order_id: Option<OrderId>,
    pub markets: Arc<Vec<MarketId>>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Pack,
    FromValue,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum AlgoKind {
    MarketMaker,
    Pov,
    SmartOrderRouter,
    Twap,
    Spread, // don't use this, use Spreader
    Chaser,
    TakeAndChase,
    QuoteOneSide,
    Spreader,
}

// CR-someday alee: use something more akin to the validator crate
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum AlgoControlCommand {
    Start,
    Pause,
    Stop,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoAck {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOut {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoReject {
    pub order_id: OrderId,
    pub reason: ArcStr,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoModify {
    pub order_id: OrderId,
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
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum AlgoRunningStatus {
    Running,
    Paused,
    Done,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoStatus {
    pub order_id: OrderId,
    pub creation_time: DateTime<Utc>,
    pub status: (DateTime<Utc>, AlgoRunningStatus),
    pub fraction_complete: Option<f64>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoLog {
    pub order_id: OrderId,
    pub fills: Arc<Vec<Result<Fill, AberrantFill>>>,
    pub rejects: Arc<Vec<Reject>>,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChildAck {
    pub algo_order_id: OrderId,
    pub ack: Ack,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChildOut {
    pub algo_order_id: OrderId,
    pub out: Out,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChildReject {
    pub algo_order_id: OrderId,
    pub reject: Reject,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChildFill {
    pub algo_order_id: OrderId,
    pub fill: Result<Fill, AberrantFill>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct AlgoPreview {
    pub orders: Arc<Vec<(DateTime<Utc>, Order)>>,
}

// support trivial casing for AlgoContainerMessage: TryInto<AlgoMessage>

impl Into<AlgoOrder> for &AlgoOrder {
    fn into(self) -> AlgoOrder {
        self.clone()
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
            | AlgoMessage::GetAlgoOrder(uuid, _)
            | AlgoMessage::GetAlgoLog(uuid, _) => Some(*uuid),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            AlgoMessage::PreviewAlgoResponse(uuid, _)
            | AlgoMessage::GetAlgoStatusResponse(uuid, _)
            | AlgoMessage::GetAlgoOrderResponse(uuid, _)
            | AlgoMessage::GetAlgoLogResponse(uuid, _) => Some(*uuid),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct NoModification {
    pub order_id: OrderId,
}
impl Into<AlgoModify> for &NoModification {
    fn into(self) -> AlgoModify {
        AlgoModify { order_id: self.order_id }
    }
}
