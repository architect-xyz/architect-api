use super::*;
use crate::{
    orderflow::{OrderType, TimeInForce},
    symbology::ExecutionVenue,
    AccountIdOrName, Dir,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseAtTime;

impl Algo for ReleaseAtTime {
    const NAME: &'static str = "RELEASE_AT_TIME";

    type Params = ReleaseAtTimeParams;
    type Status = ReleaseAtTimeStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseAtTimeParams {
    pub symbol: String,
    pub execution_venue: Option<ExecutionVenue>,
    pub account: Option<AccountIdOrName>,
    pub dir: Dir,
    pub quantity: Decimal,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub at_time: DateTime<Utc>,
}

impl DisplaySymbols for ReleaseAtTimeParams {
    fn display_symbols(&self) -> Option<Vec<String>> {
        Some(vec![self.symbol.clone()])
    }
}

impl Validate for ReleaseAtTimeParams {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseAtTimeStatus {}
