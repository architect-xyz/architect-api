use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir,
    HumanDuration, OrderId, Str,
};
use anyhow::bail;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub type PovAlgoMessage =
    AlgoContainerMessage<PovAlgoOrder, AlgoPreview, PovAlgoStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct PovAlgoOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub target_volume_frac: Decimal,
    pub min_order_quantity: Decimal,
    pub total_quantity: Decimal,
    pub end_time: DateTime<Utc>,
    pub account: Option<AccountId>,
    pub order_lockout: HumanDuration,
    pub take_through_frac: Option<Decimal>,
}

impl Into<AlgoOrder> for &PovAlgoOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            algo: Str::try_from("POV").unwrap(), // won't panic
        }
    }
}

impl Validate for PovAlgoOrder {
    fn validate(&self) -> Result<()> {
        if self.target_volume_frac < dec!(0.0001) || self.target_volume_frac > dec!(0.25)
        {
            bail!("target_volume_frac must be between 1bp and 25%");
        }
        if !self.min_order_quantity.is_sign_positive() {
            bail!("min_order_quantity must be positive");
        }
        if !self.total_quantity.is_sign_positive() {
            bail!("total_quantity must be positive");
        }
        if self.order_lockout.num_milliseconds() < 500
            || self.order_lockout.num_seconds() > 300
        {
            bail!("reject_lockout must be between 0.5 seconds and 300 seconds");
        }
        if let Some(take_through_frac) = self.take_through_frac {
            if take_through_frac.is_sign_negative() || take_through_frac > dec!(0.05) {
                bail!("take_through_frac must be between 0 and 5%");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct PovAlgoStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub market_volume: Decimal,
    pub realized_volume_frac: Option<Decimal>,
    pub quantity_filled: Decimal,
}

impl TryInto<AlgoStatus> for &PovAlgoStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}
