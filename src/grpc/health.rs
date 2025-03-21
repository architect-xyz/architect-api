use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[grpc(
    package = "json.architect",
    service = "Health",
    name = "check",
    response = "HealthCheckResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HealthCheckRequest {
    /// The service to check status for; if not provided,
    /// status of the queried server overall is returned.
    ///
    /// Generally, this will only be set when querying
    /// the API gateway.  It's not recommended to rely on
    /// internal subservice names being stable.
    pub service: Option<String>,
    pub include_metrics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HealthCheckResponse {
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BTreeMap<String, HealthMetric>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthStatus {
    Unknown,
    Serving,
    NotServing,
    ServiceUnknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct HealthMetric {
    pub timestamp: i32,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_greater_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_less_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_greater_than_or_equal_to: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_be_less_than_or_equal_to: Option<f64>,
}
