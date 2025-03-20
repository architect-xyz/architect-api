use crate::{
    orderflow::{AberrantFill, Fill, Order},
    symbology::{ExecutionVenue, Product, TradableProduct},
    AccountId, AccountIdOrName, OrderId, TraderIdOrEmail,
};
use chrono::{DateTime, Utc};
use derive::grpc;
use derive_more::{Deref, DerefMut};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[grpc(package = "json.architect")]
#[grpc(service = "Folio", name = "account_summary", response = "AccountSummary")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummaryRequest {
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
    pub trader: Option<TraderIdOrEmail>,
    /// If trader and accounts are both None, return all accounts for the user
    #[serde(default)]
    pub accounts: Option<Vec<AccountIdOrName>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummariesResponse {
    pub account_summaries: Vec<AccountSummary>,
}

#[skip_serializing_none]
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountSummary {
    pub account: AccountId,
    pub timestamp: DateTime<Utc>,
    pub balances: AccountBalances,
    pub positions: AccountPositions,
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub statistics: AccountStatistics,
}

impl AccountSummary {
    pub fn new(account: AccountId, timestamp: DateTime<Utc>) -> Self {
        Self {
            account,
            timestamp,
            balances: BTreeMap::new(),
            positions: BTreeMap::new(),
            statistics: AccountStatistics::default(),
        }
    }
}

pub type AccountBalances = BTreeMap<Product, Decimal>;
pub type AccountPositions = BTreeMap<TradableProduct, Vec<AccountPosition>>;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountStatistics {
    /// Total account equity; net liquidation value.
    pub equity: Option<Decimal>,
    /// Margin requirement calculated for worst-case based on open positions and working orders.
    pub total_margin: Option<Decimal>,
    /// Margin requirement based on current positions only.
    pub position_margin: Option<Decimal>,
    // CR alee: rename to withdrawable_cash
    /// Cash available to withdraw.
    pub cash_excess: Option<Decimal>,
    /// Total purchasing power; post-multiplied.
    /// (e.g. for cash margin account could be 2x available cash)
    pub purchasing_power: Option<Decimal>,
    pub unrealized_pnl: Option<Decimal>,
    pub realized_pnl: Option<Decimal>,
    /// Yesterday total account equity.
    pub yesterday_equity: Option<Decimal>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AccountPosition {
    pub quantity: Decimal,
    /// NB: the meaning of this field varies by reporting venue
    pub trade_time: Option<DateTime<Utc>>,
    /// Cost basis of the position, if known.
    pub cost_basis: Option<Decimal>,
    /// Unrealized PNL of the position, if known.
    pub unrealized_pnl: Option<Decimal>,
    pub break_even_price: Option<Decimal>,
    pub liquidation_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountHistoryRequest {
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
    pub trader: Option<TraderIdOrEmail>,
    pub order_id: Option<OrderId>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
    /// Default maximum is 1000.
    pub limit: Option<u32>,
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
    /// if order_ids is not empty, the limit field is ignored
    pub order_ids: Option<Vec<OrderId>>,
    pub venue: Option<ExecutionVenue>,
    pub account: Option<AccountIdOrName>,
    pub trader: Option<TraderIdOrEmail>,
    pub parent_order_id: Option<OrderId>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
    /// Default maximum is 1000.
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalOrdersResponse {
    pub orders: Vec<Order>,
}
