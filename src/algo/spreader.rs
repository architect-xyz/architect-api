use super::*;
use crate::{
    symbology::{ExecutionVenue, MarketdataVenue},
    AccountIdOrName, Dir, HumanDuration,
};
use anyhow::{bail, Result};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Spreader;

impl Algo for Spreader {
    const NAME: &'static str = "SPREADER";

    type Params = SpreaderParams;
    type Status = SpreaderStatus;
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpreaderParams {
    pub dir: Dir,
    pub quantity: Decimal,
    pub limit_price: Decimal,
    pub order_lockout: HumanDuration,
    pub leg1_symbol: String,
    pub leg1_account: Option<AccountIdOrName>,
    pub leg1_marketdata_venue: MarketdataVenue,
    pub leg1_execution_venue: Option<ExecutionVenue>,
    pub leg1_price_ratio: Decimal,
    pub leg1_price_offset: Decimal,
    pub leg1_quantity_ratio: Decimal,
    pub leg2_symbol: String,
    pub leg2_account: Option<AccountIdOrName>,
    pub leg2_marketdata_venue: MarketdataVenue,
    pub leg2_execution_venue: Option<ExecutionVenue>,
    pub leg2_price_ratio: Decimal,
    pub leg2_price_offset: Decimal,
    pub leg2_quantity_ratio: Decimal,
}

impl DisplaySymbols for SpreaderParams {
    fn display_symbols(&self) -> Option<Vec<String>> {
        Some(vec![self.leg1_symbol.clone(), self.leg2_symbol.clone()])
    }
}

impl Validate for SpreaderParams {
    fn validate(&self) -> Result<()> {
        // Must ensure that quantity and various ratios are non-zero or else we'd have division by zero.
        if self.leg1_price_ratio.is_zero() {
            bail!("leg1_price_ratio must not be zero");
        }
        if self.leg2_price_ratio.is_zero() {
            bail!("leg2_price_ratio must not be zero");
        }
        if self.leg1_quantity_ratio.is_zero() {
            bail!("leg1_quantity_ratio must not be zero");
        }
        if self.leg2_quantity_ratio.is_zero() {
            bail!("leg2_quantity_ratio must not be zero");
        }
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if self.order_lockout.is_zero() {
            bail!("order_lockout must not be zero");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SpreaderStatus {
    pub leg1_fill_quantity: Decimal,
    pub leg2_fill_quantity: Decimal,
    pub implied_spread_vwap: Option<Decimal>,
    pub current_spreader_phase: SpreaderPhase,
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, Default, JsonSchema, PartialEq, Eq,
)]
pub enum SpreaderPhase {
    #[default]
    ScanningForTakes,
    AwaitingOrderResults,
    OrderLockout,
    NoBbo,
    NotEnoughBboSize,
    DoneOverfilled,
    DoneAndFullyHedged,
    DoneAndGivingUp,
}
