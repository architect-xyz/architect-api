use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir,
    HumanDuration, OrderId,
};
use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type TakeAndChaseAlgoMessage = AlgoContainerMessage<
    TakeAndChaseOrder,
    TakeAndChaseAlgoModify,
    AlgoPreview,
    TakeAndChaseStatus,
    AlgoLog,
>;

/// An advanced algorithm that starts by attempting to cross a limit order at a given price
/// and then, if necessary, incrementally makes the order’s price more aggressive by
/// “chasing” the best available quote on the opposite side up to the specified limit_price.
/// (It sends a limit order at the best price on the opposite side of the book)
/// This allows the order to become more aggressive if it fails to fill immediately.
///
/// Key functionality:
/// - The algorithm places the order at the best displayed price on the opposite side
///   of the book, as long as moving to that price stays within the defined limit_price.
///
/// - If `dont_chase_if_opposite_side_qty_too_large` is set, the algorithm will not cross
///   unless the displayed quantity on the opposite side falls below this threshold.
///   If the opposite side has more quantity than the threshold, the order
///   the more aggressive of the set price and resting at the top of its own side
///   until the condition is met.
///   This can be useful to avoid becoming overly aggressive when facing large resting
///   liquidity on the opposite side. It is recommended that this threshold be either None
///   or much larger than the order quantity.
///
/// - Once the order reaches the most aggressive allowable price without filling (i.e.,
///   the limit_price), it enters a "at_limit_price" state. In this state, the
///   order remains resting at the final, most aggressive price and does not continue
///   to get more aggressive.
///
/// - If a `lockout` duration is specified, once the order has its price modified,
///   the algorithm will not modify it again until the lockout period has elapsed.
///
/// - If `start_maximally_aggressive` is set, the order will be sent at the limit_price
///   (instead of just the opposite side price) if the opposite side quantity threshold
///   is not active.
///
/// - If `dont_chase_if_opposite_side_qty_too_large` = None and
///   `start_maximally_agressive``=true, this is essentially a limit order.
///   
/// - If `dont_chase_if_opposite_side_qty_too_large`=Some(qty) and start
///   `maximally_aggressive=true, then try to start at the max price but if the opposite
///   quantity is too large, join the best order on the same side, but then send the
///   maximally aggressive price when the opposite side quantity falls below the threshold.
///
///  - There is a possibility of overfill if either Architect or the exchange does not
///    support the modification of orders. This would happen in the midst of cancelling
///    the order and sending a new one.
///
#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct TakeAndChaseOrder {
    // CR acho: allow for market aggregates (one product over all exchanges)
    pub order_id: OrderId,
    pub market: MarketId,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub dir: Dir,
    pub quantity: Decimal,
    pub limit_price: Decimal,
    pub dont_chase_if_opposite_side_qty_too_large: Option<Decimal>,
    pub start_maximally_aggressive: bool,
    pub lockout: Option<HumanDuration>,
    pub parent_order_id: Option<OrderId>,
}

impl TakeAndChaseOrder {
    pub fn modify(&mut self, modify: &TakeAndChaseAlgoModify) {
        let TakeAndChaseAlgoModify { order_id: _, new_quantity, new_limit_price } =
            modify;
        if let Some(new_quantity) = new_quantity {
            self.quantity = *new_quantity;
        }
        if let Some(new_limit_price) = new_limit_price {
            self.limit_price = *new_limit_price;
        }
    }
}

impl Into<AlgoOrder> for &TakeAndChaseOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo: AlgoKind::TakeAndChase,
            parent_order_id: self.parent_order_id,
            markets: Arc::new(vec![self.market]),
        }
    }
}

impl Validate for TakeAndChaseOrder {
    fn validate(&self) -> Result<()> {
        if self
            .dont_chase_if_opposite_side_qty_too_large
            .is_some_and(|threshold| !threshold.is_sign_positive())
        {
            bail!("dont_chase_if_opposite_side_qty_too_large must be positive");
        }
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if let Some(lockout) = self.lockout {
            if lockout.num_milliseconds() < 0 {
                bail!("lockout must be positive");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct TakeAndChaseStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub realized_avg_price: Option<Decimal>,
    pub quantity_filled: Decimal,
    pub at_limit_price: bool,
}

impl TryInto<AlgoStatus> for &TakeAndChaseStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct TakeAndChaseAlgoModify {
    pub order_id: OrderId,
    pub new_quantity: Option<Decimal>,
    pub new_limit_price: Option<Decimal>,
}

impl Into<AlgoModify> for &TakeAndChaseAlgoModify {
    fn into(self) -> AlgoModify {
        AlgoModify { order_id: self.order_id }
    }
}
