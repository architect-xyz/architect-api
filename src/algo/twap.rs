use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir, OrderId, Str,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
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
    pub account: Option<AccountId>,
}

impl Into<AlgoOrder> for &TwapOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            algo: Str::try_from("TWAP").unwrap(), // won't panic
        }
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
