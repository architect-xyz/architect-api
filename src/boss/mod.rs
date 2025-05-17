use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod protocol;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Statement {
    pub statement_uuid: Uuid,
    pub account: String,
    pub statement_type: String,
    pub clearing_firm: String,
    pub statement_date: NaiveDate,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Deposit {
    pub account: String,
    pub amount: Decimal,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Withdrawal {
    pub account: String,
    pub amount: Decimal,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct RqdAccountStatistics {
    pub account_number: String,
    pub available_cash_balance: Option<Decimal>,
    pub account_type: Option<String>,
    pub overnight_buying_power: Option<Decimal>,
    pub cash_available_for_withdrawal: Option<Decimal>,
    pub cumulative_day_trade_pnl: Option<Decimal>,
    pub is_day_trader: Option<String>,
    pub day_trading_buying_power: Option<Decimal>,
    pub day_trading_margin_call_amount: Option<Decimal>,
    pub day_trading_buying_power_high_water_mark: Option<Decimal>,
    pub start_of_day_day_trading_buying_power: Option<Decimal>,
    pub day_trading_buying_power_maintenance_margin_multiplier: Option<Decimal>,
    pub day_trading_minimum_equity_margin_call_amount: Option<Decimal>,
    pub day_trading_house_minimum_equity_margin_call_amount: Option<Decimal>,
    pub total_equity: Option<Decimal>,
    pub house_margin_requirement_adjustment_factor: Option<Decimal>,
    pub margin_call_amount: Option<Decimal>,
    pub maintenance_margin_requirement: Option<Decimal>,
    pub excess_sma_amount: Option<Decimal>,
    pub reg_t_margin_call_amount: Option<Decimal>,
    pub reg_t_initial_margin_requirement: Option<Decimal>,
    pub house_initial_margin_requirement_adjustment_factor: Option<Decimal>,
    pub house_margin_call_amount: Option<Decimal>,
    pub house_margin_requirement: Option<Decimal>,
    pub trade_date_total_long_market_value: Option<Decimal>,
    pub market_value_adjustment_factor: Option<Decimal>,
    pub marginable_equity: Option<Decimal>,
    pub trade_date_option_long_market_value: Option<Decimal>,
    pub number_open_day_trading_margin_calls: Option<Decimal>,
    pub start_of_day_day_trading_buying_power_margin_call_amount: Option<Decimal>,
    pub start_of_day_maintenance_margin_call_amount: Option<Decimal>,
    pub start_of_day_reg_t_margin_call_amount: Option<Decimal>,
    pub start_of_day_house_margin_call_amount: Option<Decimal>,
    pub usable_sma_balance: Option<Decimal>,
    pub option_only_maintenance_margin_requirement: Option<Decimal>,
    pub option_trade_date_short_market_value: Option<Decimal>,
    pub reg_t_maintenance_margin_requirement_adjustment_factor: Option<String>,
    pub strategy_based_relief_adjustment_factor: Option<Decimal>,
    pub settlement_date_cash_balance: Option<Decimal>,
    pub settlement_date_long_market_value: Option<Decimal>,
    pub settlement_date_option_long_market_value: Option<Decimal>,
    pub settlement_date_option_short_market_value: Option<Decimal>,
    pub settlement_date_short_market_value: Option<Decimal>,
    pub overall_current_sma_balance: Option<Decimal>,
    pub trade_date_total_short_market_value: Option<Decimal>,
    pub as_of_date: Option<String>,
    pub trade_date_cash_balance: Option<Decimal>,
}
