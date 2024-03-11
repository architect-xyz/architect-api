use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir, HumanDuration, OrderId, Str
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type PovAlgoMessage = AlgoContainerMessage<PovAlgoOrder, AlgoPreview, PovAlgoStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct PovAlgoOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub target_volume_frac: Decimal,
    pub min_order_quantity: Decimal,
    pub max_quantity: Decimal,
    pub end_time: DateTime<Utc>,
    pub account: Option<Str>,
    pub order_lockout: HumanDuration,
}

impl Into<AlgoOrder> for &PovAlgoOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            algo: Str::try_from("POV").unwrap(), // won't panic
        }
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

