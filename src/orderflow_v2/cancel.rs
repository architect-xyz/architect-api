use crate::{symbology_v2::ExecutionVenue, OrderId};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CancelOrderRequest {
    pub id: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    #[schemars(with = "OrderId")]
    pub order_id: OrderId,
}

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

#[serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct Cancel {
    pub id: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    #[schemars(with = "OrderId")]
    pub order_id: OrderId,
    pub recv_time: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CancelAck {
    pub id: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    #[schemars(with = "OrderId")]
    pub order_id: OrderId,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelReject {
    pub id: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    #[schemars(with = "OrderId")]
    pub order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
