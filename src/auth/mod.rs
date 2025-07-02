use crate::UserId;
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod grants;
pub mod jwt;
pub mod user_id;

pub use grants::Grants;

/// Create a session JWT to use for authentication with upstream gRPC services.
///
/// If grants are not specified, the JWT will be created with the same grants as the API key.
#[grpc(package = "json.architect")]
#[grpc(service = "Auth", name = "create_jwt", response = "CreateJwtResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateJwtRequest {
    pub api_key: String,
    pub api_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grants: Option<Grants>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateJwtResponse {
    pub jwt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthInfoRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthInfoResponse {
    pub user_id: Option<UserId>,
    pub original_user_id: Option<UserId>,
}
