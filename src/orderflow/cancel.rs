use crate::OrderId;
use chrono::{DateTime, Utc};
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Cancel {
    #[serde(rename = "xid")]
    pub cancel_id: Uuid,
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "ts")]
    pub recv_time: i64,
    #[serde(rename = "tn")]
    pub recv_time_ns: u32,
    #[serde(rename = "o")]
    pub status: CancelStatus,
    #[serde(rename = "r")]
    pub reject_reason: Option<String>,
}

impl Cancel {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
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
