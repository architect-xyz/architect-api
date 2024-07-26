use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, DirPair,
    HumanDuration, OrderId,
};
use anyhow::bail;
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub type MMAlgoMessage =
    AlgoContainerMessage<MMAlgoOrder, AlgoPreview, MMAlgoStatus, AlgoLog>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum ReferencePrice {
    Mid,
    BidAsk,
    HedgeMarketBidAsk,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct HedgeMarket {
    pub market: MarketId,
    pub conversion_ratio: Decimal,
    pub premium: Decimal,
    pub hedge_frac: Decimal,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct MMAlgoOrder {
    pub order_id: OrderId,
    pub market: MarketId,
    pub trader: UserId,
    pub account: Option<AccountId>,
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
    pub hedge_market: Option<HedgeMarket>,
    pub parent_order_id: Option<OrderId>,
}

impl Into<AlgoOrder> for &MMAlgoOrder {
    fn into(self) -> AlgoOrder {
        let algo = if self.hedge_market.is_some() {
            AlgoKind::Spread
        } else {
            AlgoKind::MarketMaker
        };
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo,
            parent_order_id: self.parent_order_id,
        }
    }
}

impl Validate for MMAlgoOrder {
    fn validate(&self) -> Result<()> {
        if !self.quantity.buy.is_sign_positive() {
            bail!("quantity.buy must be positive");
        }
        if !self.quantity.sell.is_sign_positive() {
            bail!("quantity.sell must be positive");
        }
        if self.min_position >= self.max_position {
            bail!("min_position must be < max_position");
        }
        if self.position_tilt.is_sign_negative() {
            bail!("position_tilt must be non-negative");
        }
        if self.ref_dist_frac < dec!(0.00001) || self.ref_dist_frac > dec!(0.25) {
            bail!("ref_dist_frac must be between 0.1bp and 25%");
        }
        if self.tolerance_frac < dec!(0.00001) || self.tolerance_frac > dec!(0.25) {
            bail!("tolerance_frac must be between 0.1bp and 25%");
        }
        if self.reject_lockout.num_milliseconds() < 500
            || self.reject_lockout.num_seconds() > 300
        {
            bail!("reject_lockout must be between 0.5 seconds and 300 seconds");
        }
        if self.order_lockout.num_milliseconds() < 500
            || self.order_lockout.num_seconds() > 300
        {
            bail!("order_lockout must be between 0.5 seconds and 300 seconds");
        }
        if self.fill_lockout.num_milliseconds() < 500
            || self.fill_lockout.num_seconds() > 300
        {
            bail!("fill_lockout must be between 0.5 seconds and 300 seconds");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct MMAlgoStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub position: Decimal,
    pub hedge_position: Decimal,
    pub sides: DirPair<Side>,
    pub kind: MMAlgoKind,
}

impl TryInto<AlgoStatus> for &MMAlgoStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum MMAlgoKind {
    MM,
    Spread,
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
    AlgoPaused,
    AlgoStopped,
    MinPosition,
    MaxPosition,
    WithinFillLockout,
    WithinRejectLockout,
    WithinOrderLockout,
    NoReferencePrice,
    NoReferenceSize,
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
    pub reference_price: Option<Decimal>,
}

impl Side {
    pub fn new() -> Self {
        Self {
            last_decision: Decision::DoNothing(vec![]),
            last_order_time: DateTime::<Utc>::MIN_UTC,
            last_fill_time: DateTime::<Utc>::MIN_UTC,
            last_reject_time: DateTime::<Utc>::MIN_UTC,
            open_order: None,
            reference_price: None,
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
