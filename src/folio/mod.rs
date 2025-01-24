use crate::{
    orderflow::{AberrantFill, Fill, Order},
    symbology::{ExecutionVenue, Product},
    AccountId, OrderId, UserId,
};
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[grpc(package = "json.architect")]
#[grpc(service = "Folio", name = "account_summary", response = "AccountSummary")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummaryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<ExecutionVenue>,
    pub account: AccountId,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Folio",
    name = "account_summaries",
    response = "AccountSummariesResponse"
)]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummariesRequest {
    #[serde(default)]
    pub venue: Option<ExecutionVenue>,
    #[serde(default)]
    pub trader: Option<UserId>,
    /// If not provided, all accounts for venue will be returned.
    #[serde(default)]
    pub accounts: Option<Vec<AccountId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummariesResponse {
    pub account_summaries: Vec<AccountSummary>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummary {
    pub account: AccountId,
    pub timestamp: DateTime<Utc>,
    pub balances: BTreeMap<Product, Decimal>,
    pub positions: Vec<AccountPosition>,
    pub unrealized_pnl: Option<Decimal>,
    pub realized_pnl: Option<Decimal>,
    pub equity: Option<Decimal>,
    pub yesterday_equity: Option<Decimal>,
    pub cash_excess: Option<Decimal>,
    pub total_margin: Option<Decimal>,
    pub position_margin: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AccountPosition {
    pub account: AccountId,
    pub symbol: String,
    pub quantity: Decimal,
    /// The meaning of this field varies by reporting venue.
    pub trade_time: Option<DateTime<Utc>>,
    pub cost_basis: Decimal,
    pub break_even_price: Option<Decimal>,
    pub liquidation_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountHistoryRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: AccountId,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountHistoryResponse {
    pub history: Vec<AccountSummary>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Folio",
    name = "historical_fills",
    response = "HistoricalFillsResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalFillsRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountId>,
    pub order_id: Option<OrderId>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalFillsResponse {
    pub fills: Vec<Fill>,
    pub aberrant_fills: Vec<AberrantFill>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Folio",
    name = "historical_orders",
    response = "HistoricalOrdersResponse"
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalOrdersRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountId>,
    pub parent_order_id: Option<OrderId>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalOrdersResponse {
    pub orders: Vec<Order>,
}
