use super::Statement;
use crate::AccountId;
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[grpc(package = "json.architect")]
#[grpc(service = "Boss", name = "statements", response = "StatementsResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatementsRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatementsResponse {
    pub statements: Vec<Statement>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Boss", name = "statement_url", response = "StatementUrlResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatementUrlRequest {
    pub statement_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatementUrlResponse {
    pub statement_url: String,
}
