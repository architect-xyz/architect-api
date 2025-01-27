use crate::{oms::*, symbology::ExecutionVenue, AccountId, UserId};
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
#[serde(tag = "t")]
pub enum OrderflowRequest {
    #[serde(rename = "p")]
    PlaceOrder(PlaceOrderRequest),
    #[serde(rename = "x")]
    CancelOrder(CancelOrderRequest),
    #[serde(rename = "xo")]
    CancelAllOrders(CancelAllOrdersRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
pub enum OrderflowResponse {
    #[serde(rename = "w")]
    OrderPending(Order),
    #[serde(rename = "a")]
    OrderAck(OrderAck),
    #[serde(rename = "r")]
    OrderReject(OrderReject),
    #[serde(rename = "o")]
    OrderOut(OrderOut),
    #[serde(rename = "z")]
    OrderStale(OrderStale),
    #[serde(rename = "xc")]
    CancelPending(Cancel),
    #[serde(rename = "xa")]
    CancelAck(CancelAck),
    #[serde(rename = "xr")]
    CancelReject(CancelReject),
    #[serde(rename = "f")]
    Fill(Fill),
    #[serde(rename = "af")]
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
#[serde(tag = "t")]
pub enum DropcopyResponse {
    #[serde(rename = "o")]
    Order(Order),
    #[serde(rename = "f")]
    Fill(Fill),
    #[serde(rename = "af")]
    AberrantFill(AberrantFill),
}
