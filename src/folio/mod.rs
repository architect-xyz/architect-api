use crate::{
    orderflow::{AberrantFill, Fill, Order},
    symbology::{ExecutionVenue, Product, TradableProduct},
    AccountId, AccountIdOrName, OrderId, TraderIdOrEmail,
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
    pub account: AccountIdOrName,
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
    pub trader: Option<TraderIdOrEmail>,
    /// If not provided, all accounts for venue will be returned.
    #[serde(default)]
    pub accounts: Option<Vec<AccountIdOrName>>,
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
    pub positions: BTreeMap<TradableProduct, Vec<AccountPosition>>,
    pub equity: Option<Decimal>,
    /// Margin requirement calculated for worst-case based on open positions and working orders.
    pub total_margin: Option<Decimal>,
    /// Margin requirement based on current positions only.
    pub position_margin: Option<Decimal>,
    /// Cash available to withdraw.
    pub cash_excess: Option<Decimal>,
    pub purchasing_power: Option<Decimal>,
    pub unrealized_pnl: Option<Decimal>,
    pub realized_pnl: Option<Decimal>,
    pub yesterday_equity: Option<Decimal>,
}

impl AccountSummary {
    pub fn new(account: AccountId, timestamp: DateTime<Utc>) -> Self {
        Self {
            account,
            timestamp,
            balances: BTreeMap::new(),
            positions: BTreeMap::new(),
            equity: None,
            total_margin: None,
            position_margin: None,
            cash_excess: None,
            purchasing_power: None,
            unrealized_pnl: None,
            realized_pnl: None,
            yesterday_equity: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AccountPosition {
    pub quantity: Decimal,
    /// The meaning of this field varies by reporting venue.
    pub trade_time: Option<DateTime<Utc>>,
    pub cost_basis: Option<Decimal>,
    pub break_even_price: Option<Decimal>,
    pub liquidation_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountHistoryRequest {
    pub venue: Option<ExecutionVenue>,
    pub account: AccountIdOrName,
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
    pub account: Option<AccountIdOrName>,
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
    pub account: Option<AccountIdOrName>,
    pub parent_order_id: Option<OrderId>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalOrdersResponse {
    pub orders: Vec<Order>,
}
