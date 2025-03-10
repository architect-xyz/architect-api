use crate::{oms::*, symbology::ExecutionVenue, AccountIdOrName, TraderIdOrEmail};
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
#[grpc(service = "Orderflow", name = "orderflow", response = "Orderflow", bidi_streaming)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
/// <!-- py: tag=t -->
pub enum OrderflowRequest {
    #[serde(rename = "p")]
    #[schemars(title = "PlaceOrder|PlaceOrderRequest")]
    PlaceOrder(PlaceOrderRequest),
    #[serde(rename = "x")]
    #[schemars(title = "CancelOrder|CancelOrderRequest")]
    CancelOrder(CancelOrderRequest),
    #[serde(rename = "xo")]
    #[schemars(title = "CancelAllOrders|CancelAllOrdersRequest")]
    CancelAllOrders(CancelAllOrdersRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
/// <!-- py: tag=t -->
pub enum Orderflow {
    #[serde(rename = "w")]
    #[schemars(title = "OrderPending|Order")]
    OrderPending(Order),
    #[serde(rename = "a")]
    #[schemars(title = "OrderAck|OrderAck")]
    OrderAck(OrderAck),
    #[serde(rename = "r")]
    #[schemars(title = "OrderReject|OrderReject")]
    OrderReject(OrderReject),
    #[serde(rename = "o")]
    #[schemars(title = "OrderOut|OrderOut")]
    OrderOut(OrderOut),
    #[serde(rename = "ox")]
    #[schemars(title = "OrderReconciledOut|OrderOut")]
    OrderReconciledOut(OrderOut),
    #[serde(rename = "z")]
    #[schemars(title = "OrderStale|OrderStale")]
    OrderStale(OrderStale),
    #[serde(rename = "xc")]
    #[schemars(title = "CancelPending|Cancel")]
    CancelPending(Cancel),
    #[serde(rename = "xr")]
    #[schemars(title = "CancelReject|CancelReject")]
    CancelReject(CancelReject),
    #[serde(rename = "xa")]
    #[schemars(title = "OrderCanceling|OrderCanceling")]
    OrderCanceling(OrderCanceling),
    #[serde(rename = "xx")]
    #[schemars(title = "OrderCanceled|OrderCanceled")]
    OrderCanceled(OrderCanceled),
    #[serde(rename = "f")]
    #[schemars(title = "Fill|Fill")]
    Fill(Fill),
    #[serde(rename = "af")]
    #[schemars(title = "AberrantFill|AberrantFill")]
    AberrantFill(AberrantFill),
}

/// Subscribe/listen to orderflow events.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Orderflow",
    name = "subscribe_orderflow",
    response = "Orderflow",
    server_streaming
)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeOrderflowRequest {
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
    #[serde(default)]
    pub trader: Option<TraderIdOrEmail>,
    #[serde(default)]
    pub account: Option<AccountIdOrName>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Orderflow", name = "dropcopy", response = "Dropcopy", server_streaming)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DropcopyRequest {
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
    #[serde(default)]
    pub trader: Option<TraderIdOrEmail>,
    #[serde(default)]
    pub account: Option<AccountIdOrName>,
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
/// <!-- py: tag=t -->
pub enum Dropcopy {
    #[serde(rename = "o")]
    #[schemars(title = "Order|Order")]
    Order(Order),
    #[schemars(title = "Fill|Fill")]
    #[serde(rename = "f")]
    Fill(Fill),
    #[serde(rename = "af")]
    #[schemars(title = "AberrantFill|AberrantFill")]
    AberrantFill(AberrantFill),
}
