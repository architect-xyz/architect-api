use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(
    Debug, Clone, Copy, IntoStaticStr, Serialize, Deserialize, PartialEq, Eq, JsonSchema,
)]
#[serde(tag = "k", rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit(LimitOrderType),
    StopLossLimit(TriggerLimitOrderType),
    TakeProfitLimit(TriggerLimitOrderType),
}

impl OrderType {
    pub fn limit_price(&self) -> Option<Decimal> {
        match self {
            OrderType::Limit(limit) => Some(limit.limit_price),
            OrderType::StopLossLimit(stop_loss) => Some(stop_loss.limit_price),
            OrderType::TakeProfitLimit(take_profit) => Some(take_profit.limit_price),
            OrderType::Market => None,
        }
    }

    pub fn post_only(&self) -> Option<bool> {
        match self {
            OrderType::Limit(limit) => Some(limit.post_only),
            OrderType::StopLossLimit(_) => None,
            OrderType::TakeProfitLimit(_) => None,
            OrderType::Market => None,
        }
    }

    pub fn trigger_price(&self) -> Option<Decimal> {
        match self {
            OrderType::Limit(_) => None,
            OrderType::StopLossLimit(stop_loss) => Some(stop_loss.trigger_price),
            OrderType::TakeProfitLimit(take_profit) => Some(take_profit.trigger_price),
            OrderType::Market => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct LimitOrderType {
    #[serde(rename = "p")]
    #[schemars(title = "limit_price")]
    pub limit_price: Decimal,
    #[serde(rename = "po")]
    #[schemars(title = "post_only")]
    pub post_only: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct TriggerLimitOrderType {
    #[serde(rename = "p")]
    #[schemars(title = "limit_price")]
    pub limit_price: Decimal,
    #[serde(rename = "tp")]
    #[schemars(title = "trigger_price")]
    pub trigger_price: Decimal,
}
