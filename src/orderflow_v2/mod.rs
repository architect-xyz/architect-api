use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod cancel;
pub mod fill;
pub mod order;

pub use cancel::*;
pub use fill::*;
pub use order::*;

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
    PlaceOrder(PlaceOrderRequest),
    CancelOrder(CancelOrderRequest),
    CancelAllOrders(CancelAllOrdersRequest),
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
