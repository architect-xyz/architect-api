use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir, OrderId,
};
use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// An advanced algo that quotes one side of a market by joining the passive side within
/// a specified number of ticks, with the option to improve the market by one tick to
/// gain queue priority.
/// The primary intended use is in the context of the spreader algo, where it will be used
/// on the passive side of a spread.
///
/// Key functionality:
/// - `max_ticks_outside` is used to determine the range of ticks from the best same side price to quote
/// `max_ticks_outside` is the maximum number of ticks outside the market to quote (ie less aggressive than bbo)
///
/// - This algo will always put out a limit order with a price that is equal to or better than the
/// set price. It will attempt to only post, so it will not cross the market unless
/// the market moves toward the order in the midst of sending the order.
///
/// - This algo will improve the market by one tick `improve` is true, the opposite side is one tick away
///

pub type QuoteOneSideAlgoMessage = AlgoContainerMessage<
    QuoteOneSideOrder,
    QuoteOneSideModify,
    AlgoPreview,
    QuoteOneSideStatus,
    AlgoLog,
>;

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSideOrder {
    // CR acho: allow for market aggregates
    // CR acho: allow for modifying the limit orders
    // CR acho: allow for a "take mode" where the algo will convert to a take_with_chase_ticks algo
    // this will require a modify limit order functionality
    pub order_id: OrderId,
    pub market: MarketId,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub dir: Dir,
    pub quantity: Decimal,
    pub limit_price: Decimal,
    pub max_ticks_outside: Option<Decimal>,
    pub improve_or_join: ImproveOrJoin,
    pub parent_order_id: Option<OrderId>,
    // pub take_using_quotes: bool, // this prevents double fill in the case of quoting two
}

impl Into<AlgoOrder> for &QuoteOneSideOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo: AlgoKind::QuoteOneSide,
            parent_order_id: self.parent_order_id,
            markets: Arc::new(vec![self.market]),
        }
    }
}

impl Validate for QuoteOneSideOrder {
    fn validate(&self) -> Result<()> {
        if let Some(max_ticks_outside) = self.max_ticks_outside {
            if !max_ticks_outside.is_sign_positive() {
                bail!("max_ticks_outside must be positive or None");
            }

            if !max_ticks_outside.is_integer() {
                bail!("max_ticks_outside must be an integer value");
            }
        }
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum ImproveOrJoin {
    Improve,
    Join,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSideStatus {
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub realized_avg_price: Option<Decimal>,
    pub quantity_filled: Decimal,
}

impl TryInto<AlgoStatus> for &QuoteOneSideStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSideModify {
    pub order_id: OrderId,
    pub quantity: Option<Decimal>,
    pub limit_price: Option<Decimal>,
}
impl Into<AlgoModify> for &QuoteOneSideModify {
    fn into(self) -> AlgoModify {
        AlgoModify { order_id: self.order_id }
    }
}

impl QuoteOneSideOrder {
    pub fn apply(&mut self, modify: &QuoteOneSideModify) {
        let QuoteOneSideModify { order_id: _, quantity, limit_price } = modify;
        if let Some(quantity) = quantity {
            self.quantity = *quantity;
        }
        if let Some(limit_price) = limit_price {
            self.limit_price = *limit_price;
        }
    }
}
