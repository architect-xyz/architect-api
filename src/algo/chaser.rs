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

pub type ChaserAlgoMessage =
    AlgoContainerMessage<ChaserOrder, NoModification, AlgoPreview, ChaserStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChaserOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub end_time: DateTime<Utc>,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub reject_lockout: Duration,
    pub take_through_frac: Option<Decimal>,
    pub parent_order_id: Option<OrderId>,
}

impl Into<AlgoOrder> for &ChaserOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo: AlgoKind::Chaser,
            parent_order_id: self.parent_order_id,
            markets: Arc::new(vec![self.market]),
        }
    }
}

impl Validate for ChaserOrder {
    fn validate(&self) -> Result<()> {
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if let Some(take_through_frac) = self.take_through_frac {
            if take_through_frac.is_sign_negative() || take_through_frac > dec!(0.05) {
                bail!("take_through_frac must be between 0 and 5%");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct ChaserStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub realized_avg_price: Option<Decimal>,
    pub quantity_filled: Decimal,
}

impl TryInto<AlgoStatus> for &ChaserStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}
