use crate::OrderId;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Cancel {
    pub id: Uuid,
    pub order_id: OrderId,
    pub recv_time: DateTime<Utc>,
    pub status: CancelStatus,
    pub reject_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum CancelStatus {
    Pending,
    Acked,
    Rejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct CancelAck {
    pub id: Uuid,
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CancelReject {
    pub id: Uuid,
    pub order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
