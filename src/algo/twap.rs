use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir, OrderId,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub type TwapMessage = AlgoContainerMessage<TwapOrder, AlgoPreview, TwapStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct TwapOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub interval: Duration,
    pub end_time: DateTime<Utc>,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub reject_lockout: Duration,
    pub take_through_frac: Option<Decimal>,
    pub parent_order_id: Option<OrderId>,
}

impl Into<AlgoOrder> for &TwapOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo: AlgoKind::Twap,
            parent_order_id: self.parent_order_id,
        }
    }
}

impl Validate for TwapOrder {
    fn validate(&self) -> Result<()> {
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if self.interval.as_secs() < 1 {
            bail!("interval must be >= 1 second");
        }
        if self.reject_lockout.as_millis() < 500 || self.reject_lockout.as_secs() > 300 {
            bail!("reject lockout must be between 0.5 seconds and 300 seconds");
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
pub struct TwapStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub realized_twap: Option<Decimal>,
    pub quantity_filled: Decimal,
}

impl TryInto<AlgoStatus> for &TwapStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}
