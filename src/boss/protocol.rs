use super::*;
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

#[grpc(package = "json.architect")]
#[grpc(
    service = "Boss",
    name = "rqd_account_statistics",
    response = "RqdAccountStatisticsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RqdAccountStatisticsRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RqdAccountStatisticsResponse {
    pub rqd_account_statistics: RqdAccountStatistics,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Boss", name = "deposits", response = "DepositsResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DepositsRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DepositsResponse {
    pub deposits: Vec<Deposit>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Boss", name = "withdrawals", response = "WithdrawalsResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WithdrawalsRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WithdrawalsResponse {
    pub withdrawals: Vec<Withdrawal>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Boss",
    name = "options_transactions",
    response = "OptionsTransactionsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsTransactionsRequest {
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsTransactionsResponse {
    pub options_transactions: Vec<OptionsTransaction>,
}
