use crate::OrderId;
use chrono::{DateTime, Utc};
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Cancel {
    #[serde(rename = "xid")]
    #[schemars(title = "cancel_id")]
    pub cancel_id: Uuid,
    #[serde(rename = "id")]
    #[schemars(title = "order_id")]
    pub order_id: OrderId,
    #[serde(rename = "ts")]
    #[schemars(title = "recv_time")]
    pub recv_time: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: u32,
    #[serde(rename = "o")]
    #[schemars(title = "status")]
    pub status: CancelStatus,
    #[serde(rename = "r")]
    #[schemars(title = "reject_reason")]
    pub reject_reason: Option<String>,
}

impl Cancel {
    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.recv_time, self.recv_time_ns)
    }
}

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, JsonSchema_repr,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum CancelStatus {
    Pending = 0,
    Acked = 1,
    Rejected = 2,
    Out = 127,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct CancelReject {
    #[serde(rename = "xid")]
    pub cancel_id: Uuid,
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "rm", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CancelReject {
    pub fn to_error_string(&self) -> String {
        format!(
            "cancel {} rejected: {}",
            self.cancel_id,
            self.message.as_deref().unwrap_or("--")
        )
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelUpdate {
    pub cancel_id: Uuid,
    pub order_id: OrderId,
    pub timestamp: i64,
    pub timestamp_ns: u32,
    pub status: Option<CancelStatus>,
    pub reject_reason: Option<String>,
}

impl CancelUpdate {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}
