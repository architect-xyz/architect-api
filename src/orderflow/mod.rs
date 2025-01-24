use crate::{symbology::ExecutionVenue, AccountId, UserId};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod cancel;
pub mod fill;
pub mod order;
pub mod order_id;
pub mod order_types;

pub use cancel::*;
pub use fill::*;
pub use order::*;
pub use order_id::*;
pub use order_types::*;

#[grpc(package = "json.architect")]
#[grpc(
    service = "Orderflow",
    name = "orderflow",
    response = "OrderflowResponse",
    bidi_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderflowRequest {
    PlaceOrder(crate::oms::PlaceOrderRequest),
    CancelOrder(crate::oms::CancelOrderRequest),
    CancelAllOrders(crate::oms::CancelAllOrdersRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderflowResponse {
    OrderPending(Order),
    OrderAck(OrderAck),
    OrderReject(OrderReject),
    OrderOut(OrderOut),
    OrderStale(OrderStale),
    CancelPending(Cancel),
    CancelAck(CancelAck),
    CancelReject(CancelReject),
    Fill(Fill),
    AberrantFill(AberrantFill),
}

/// Subscribe/listen to orderflow events.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Orderflow",
    name = "subscribe_orderflow",
    response = "OrderflowResponse",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeOrderflowRequest {
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
    #[serde(default)]
    pub trader: Option<UserId>,
    #[serde(default)]
    pub account: Option<AccountId>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Orderflow",
    name = "dropcopy",
    response = "DropcopyResponse",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropcopyRequest {
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
    #[serde(default)]
    pub trader: Option<UserId>,
    #[serde(default)]
    pub account: Option<AccountId>,
    #[serde(default)]
    pub orders: bool,
    #[serde(default = "DropcopyRequest::default_fills")]
    pub fills: bool,
    #[serde(default)]
    pub aberrant_fills: bool,
}

impl DropcopyRequest {
    fn default_fills() -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropcopyResponse {
    pub orders: Vec<Order>,
    pub fills: Vec<Fill>,
    pub aberrant_fills: Vec<AberrantFill>,
}
