use crate::{
    orderflow::{order_types::*, Cancel, Order, TimeInForce},
    symbology::ExecutionVenue,
    AccountId, Dir, OrderId, UserId,
};
use derive::grpc;
use derive_builder::Builder;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "place_order", response = "Order")]
#[derive(Builder, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PlaceOrderRequest {
    pub id: Option<OrderId>,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "d")]
    #[schemars(title = "dir")]
    pub dir: Dir,
    #[serde(rename = "q")]
    #[schemars(title = "quantity")]
    pub quantity: Decimal,
    #[serde(rename = "u", default)]
    #[schemars(title = "trader")]
    #[builder(setter(strip_option), default)]
    pub trader: Option<String>,
    #[serde(rename = "a", default)]
    #[schemars(title = "account")]
    #[builder(setter(strip_option), default)]
    pub account: Option<String>,
    #[serde(flatten)]
    pub order_type: OrderType,
    #[serde(rename = "tif")]
    #[schemars(title = "time_in_force")]
    #[builder(default = "TimeInForce::GoodTilCancel")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "x", default)]
    #[schemars(title = "execution_venue")]
    #[builder(setter(strip_option), default)]
    pub execution_venue: Option<ExecutionVenue>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "cancel_order", response = "Cancel")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CancelOrderRequest {
    /// If not specified, one will be generated for you; note, in that case,
    /// you won't know for sure if the specific request went through.
    #[serde(default)]
    pub id: Option<Uuid>,
    pub order_id: OrderId,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "cancel_all_orders", response = "CancelAllOrdersResponse")]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelAllOrdersRequest {
    pub id: Uuid,
    #[serde(default)]
    pub trader: Option<String>,
    #[serde(default)]
    pub account: Option<String>,
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
}

// CR alee: we could think of a more useful response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelAllOrdersResponse {}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "open_orders", response = "OpenOrdersResponse")]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenOrdersRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountId>,
    pub trader: Option<UserId>,
    pub symbol: Option<String>,
    pub parent_order_id: Option<OrderId>,
    pub order_ids: Option<Vec<OrderId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenOrdersResponse {
    pub open_orders: Vec<Order>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "pending_cancels", response = "PendingCancelsResponse")]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingCancelsRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountId>,
    pub trader: Option<UserId>,
    pub symbol: Option<String>,
    pub cancel_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingCancelsResponse {
    pub pending_cancels: Vec<Cancel>,
}
