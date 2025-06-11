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
    Bracket(BracketOrderType),
}

impl OrderType {
    pub fn limit_price(&self) -> Option<Decimal> {
        match self {
            OrderType::Limit(limit) => Some(limit.limit_price),
            OrderType::StopLossLimit(stop_loss) => Some(stop_loss.limit_price),
            OrderType::TakeProfitLimit(take_profit) => Some(take_profit.limit_price),
            OrderType::Market => None,
            OrderType::Bracket(bracket) => Some(bracket.limit_price),
        }
    }

    pub fn post_only(&self) -> Option<bool> {
        match self {
            OrderType::Limit(limit) => Some(limit.post_only),
            OrderType::StopLossLimit(_) => None,
            OrderType::TakeProfitLimit(_) => None,
            OrderType::Market => None,
            OrderType::Bracket(br) => Some(br.post_only),
        }
    }

    pub fn trigger_price(&self) -> Option<Decimal> {
        match self {
            OrderType::Limit(_) => None,
            OrderType::StopLossLimit(stop_loss) => Some(stop_loss.trigger_price),
            OrderType::TakeProfitLimit(take_profit) => Some(take_profit.trigger_price),
            OrderType::Market => None,
            OrderType::Bracket(bracket) => {
                bracket.stop_loss.as_ref().map(|sl| sl.trigger_price)
            }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct BracketOrderType {
    #[serde(rename = "p")]
    #[schemars(title = "limit_price")]
    pub limit_price: Decimal,
    #[serde(rename = "po")]
    #[schemars(title = "post_only")]
    pub post_only: bool,
    #[serde(rename = "tpp")]
    #[schemars(title = "take_profit_price")]
    pub take_profit_price: Option<Decimal>,

    #[serde(rename = "sl")]
    #[schemars(title = "bracket_order_stop_loss")]
    pub stop_loss: Option<TriggerLimitOrderType>,
}

impl BracketOrderType {
    pub fn has_take_profit(&self) -> bool {
        self.take_profit_price.is_some()
    }

    pub fn has_stop_loss(&self) -> bool {
        self.stop_loss.is_some()
    }
}
