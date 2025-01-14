use crate::{
    orderflow::{FillId, FillKind},
    AccountId, Dir, OrderId, UserId,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Fill {
    pub fill_id: FillId,
    pub fill_kind: FillKind,
    pub exchange_fill_id: Option<String>,
    pub order_id: Option<OrderId>,
    pub trader: Option<UserId>,
    pub account: Option<AccountId>,
    pub symbol: String,
    pub dir: Dir,
    pub quantity: Decimal,
    pub price: Decimal,
    pub fee: Option<Decimal>,
    /// Fee currency, if different from the price currency
    pub fee_currency: Option<String>,
    /// When Architect received the fill, if realtime
    pub recv_time: Option<DateTime<Utc>>,
    /// When the cpty claims the trade happened
    pub trade_time: DateTime<Utc>,
}

/// Fills which we received but couldn't parse fully,
/// return details best effort
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AberrantFill {
    pub fill_id: FillId,
    pub fill_kind: Option<FillKind>,
    pub exchange_fill_id: Option<String>,
    pub order_id: Option<OrderId>,
    pub trader: Option<UserId>,
    pub account: Option<AccountId>,
    pub symbol: Option<String>,
    pub dir: Option<Dir>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub fee_currency: Option<String>,
    pub recv_time: Option<DateTime<Utc>>,
    pub trade_time: Option<DateTime<Utc>>,
}
