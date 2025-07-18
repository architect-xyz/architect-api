use crate::{
    orderflow::{
        order_types::*, Cancel, Modify, Order, OrderReject, OrderSource, TimeInForce,
    },
    symbology::ExecutionVenue,
    AccountIdOrName, Dir, OrderId, TraderIdOrEmail,
};
use chrono::{DateTime, Utc};
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
/// <!-- py: unflatten=k/order_type/OrderType, tag=k -->
pub struct PlaceOrderRequest {
    /// If not specified, one will be generated for you; note, in that case,
    /// you won't know for sure if the specific request went through.
    pub id: Option<OrderId>,
    #[serde(rename = "pid", default)]
    #[schemars(title = "parent_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<OrderId>,
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
    pub trader: Option<TraderIdOrEmail>,
    #[serde(rename = "a", default)]
    #[schemars(title = "account")]
    #[builder(setter(strip_option), default)]
    pub account: Option<AccountIdOrName>,
    #[serde(flatten)]
    pub order_type: OrderType,
    #[serde(rename = "tif")]
    #[schemars(title = "time_in_force")]
    #[builder(default = "TimeInForce::GoodTilCancel")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "src", default)]
    #[schemars(title = "source")]
    #[builder(setter(strip_option), default)]
    pub source: Option<OrderSource>,
    #[serde(rename = "x", default)]
    #[schemars(title = "execution_venue")]
    #[builder(setter(strip_option), default)]
    pub execution_venue: Option<ExecutionVenue>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "place_batch_order", response = "PlaceBatchOrderResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlaceBatchOrderRequest {
    pub place_orders: Vec<PlaceOrderRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlaceBatchOrderResponse {
    pub pending_orders: Vec<Order>,
    pub order_rejects: Vec<OrderReject>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "cancel_order", response = "Cancel")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CancelOrderRequest {
    /// If not specified, one will be generated for you; note, in that case,
    /// you won't know for sure if the specific request went through.
    #[serde(rename = "xid", default)]
    #[schemars(title = "cancel_id")]
    pub cancel_id: Option<Uuid>,
    #[serde(rename = "id")]
    #[schemars(title = "order_id")]
    pub order_id: OrderId,
}

/// The ModifyOrderRequest will cause the order to get a new OrderId
#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "modify_order", response = "Modify")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct ModifyOrderRequest {
    /// If not specified, one will be generated for you; note, in that case,
    /// you won't know for sure if the specific request went through.
    #[serde(rename = "mid", default)]
    #[schemars(title = "modify_id")]
    pub modify_id: Option<Uuid>,

    #[serde(rename = "id")]
    #[schemars(title = "order_id")]
    pub order_id: OrderId,

    #[serde(rename = "q")]
    #[schemars(title = "new_quantity")]
    pub new_quantity: Option<Decimal>,

    #[serde(rename = "p")]
    #[schemars(title = "new_price")]
    pub new_price: Option<Decimal>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "cancel_all_orders", response = "CancelAllOrdersResponse")]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelAllOrdersRequest {
    pub id: Uuid,
    #[serde(default)]
    pub trader: Option<TraderIdOrEmail>,
    #[serde(default)]
    pub account: Option<AccountIdOrName>,
    #[serde(default)]
    pub execution_venue: Option<ExecutionVenue>,
}

// CR alee: we could think of a more useful response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelAllOrdersResponse {}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "open_orders", response = "OpenOrdersResponse")]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenOrdersRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountIdOrName>,
    pub trader: Option<TraderIdOrEmail>,
    pub symbol: Option<String>,
    pub parent_order_id: Option<OrderId>,
    pub order_ids: Option<Vec<OrderId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_inclusive: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_exclusive: Option<DateTime<Utc>>,
    #[serde(default)]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenOrdersResponse {
    pub open_orders: Vec<Order>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "pending_cancels", response = "PendingCancelsResponse")]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingCancelsRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountIdOrName>,
    pub trader: Option<TraderIdOrEmail>,
    pub symbol: Option<String>,
    pub cancel_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingCancelsResponse {
    pub pending_cancels: Vec<Cancel>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "pending_modifies", response = "PendingModifiesResponse")]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingModifiesRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountIdOrName>,
    pub trader: Option<TraderIdOrEmail>,
    pub symbol: Option<String>,
    pub modify_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PendingModifiesResponse {
    pub pending_modifies: Vec<Modify>,
}
/// Manually reconcile out orders.  Useful for clearing stuck orders
/// or stale orders when a human wants to intervene.
#[grpc(package = "json.architect")]
#[grpc(service = "Oms", name = "reconcile_out", response = "ReconcileOutResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReconcileOutRequest {
    pub order_id: Option<OrderId>,
    pub order_ids: Option<Vec<OrderId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReconcileOutResponse {}
