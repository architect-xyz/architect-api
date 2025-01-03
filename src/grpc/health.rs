use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HealthCheckResponse {
    pub status: HealthStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthStatus {
    Unknown,
    Serving,
    NotServing,
    ServiceUnknown,
}
