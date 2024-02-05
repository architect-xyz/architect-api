use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, DirPair,
    HumanDuration, OrderId, Str,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type MMAlgoMessage =
    AlgoContainerMessage<MMAlgoOrder, AlgoPreview, MMAlgoStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum ReferencePrice {
    Mid,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct MMAlgoOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub account: Option<Str>,
    pub quantity: DirPair<Decimal>,
    pub min_position: Decimal,
    pub max_position: Decimal,
    pub max_improve_bbo: Decimal,
    pub position_tilt: Decimal,
    pub reference_price: ReferencePrice,
    pub ref_dist_frac: Decimal,
    pub tolerance_frac: Decimal,
    pub fill_lockout: HumanDuration,
    pub order_lockout: HumanDuration,
    pub reject_lockout: HumanDuration,
}

impl Into<AlgoOrder> for &MMAlgoOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder { order_id: self.order_id, algo: Str::try_from("MM").unwrap() }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct MMAlgoStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub position: Decimal,
    pub sides: DirPair<Side>,
}

impl TryInto<AlgoStatus> for &MMAlgoStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum Decision {
    DoNothing(Vec<Reason>),
    Cancel(OrderId, Vec<Reason>),
    Send { price: Decimal, quantity: Decimal },
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum Reason {
    MinPosition,
    MaxPosition,
    WithinFillLockout,
    WithinRejectLockout,
    WithinOrderLockout,
    NoReferencePrice,
    NoBid,
    NoAsk,
    OpenOrderWithinTolerance,
    OpenOrderOutsideTolerance,
    CancelPending,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct Side {
    pub last_decision: Decision,
    pub last_order_time: DateTime<Utc>,
    pub last_fill_time: DateTime<Utc>,
    pub last_reject_time: DateTime<Utc>,
    pub open_order: Option<OpenOrder>,
}

impl Side {
    pub fn new() -> Self {
        Self {
            last_decision: Decision::DoNothing(vec![]),
            last_order_time: DateTime::<Utc>::MIN_UTC,
            last_fill_time: DateTime::<Utc>::MIN_UTC,
            last_reject_time: DateTime::<Utc>::MIN_UTC,
            open_order: None,
        }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct OpenOrder {
    pub order_id: OrderId,
    pub price: Decimal,
    pub quantity: Decimal,
    pub cancel_pending: bool,
}
