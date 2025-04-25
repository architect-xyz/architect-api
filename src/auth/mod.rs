use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod jwt;
pub mod user_id;

/// Create a session JWT to use for authentication with upstream gRPC services.
#[grpc(package = "json.architect")]
#[grpc(service = "Auth", name = "create_jwt", response = "CreateJwtResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateJwtRequest {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateJwtResponse {
    pub jwt: String,
}
