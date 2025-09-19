use super::*;
use crate::{
    symbology::{ExecutionVenue, MarketdataVenue},
    AccountIdOrName, Dir,
};
use anyhow::{bail, Result};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// An advanced algorithm that quotes one side of a market by joining the passive side within
/// a specified number of ticks, with the option to improve the market by one tick to
/// gain queue priority.
///
/// The primary intended use is in the context of spread trading, where it can be used
/// to work the passive side of a spread while maintaining price competitiveness.
///
/// # Key Functionality
///
/// - `max_ticks_outside` determines the range of ticks from the best same-side price to quote.
///   This is the maximum number of ticks outside (less aggressive than) the BBO that the algo will quote.
///
/// - This algo will always put out a limit order with a price that is equal to or less aggressive than the
///   set limit price. It will attempt to only post liquidity, so it will not cross the market unless
///   the market moves toward the order in the midst of sending the order.
///
/// - The algorithm can improve the market by one tick when:
///   - `improve_or_join` is set to `Improve`
///   - The opposite side is at least one tick away
///   - Improving would not violate the limit price
///
/// - This algorithm will always have at most one order at a time out.
///
/// # Quote Positioning Strategy
///
/// The algorithm uses a sophisticated positioning strategy:
/// - **Join Mode**: Places orders at the current best bid/ask on the same side
///     - IMPORTANT: The algo joins at the BBO which INCLUDES its own order once placed
///     - Once at the BBO, the order maintains that price level even if other orders cancel
///     - This means the algo won't automatically back off to less aggressive prices
///     - This behavior is a side effect of only using L1 data to determine the BBO
///     - However, it may result in the algo being alone at a price level if others cancel
///     - The order will only move to MORE aggressive prices (never less aggressive in Join mode)
/// - **Improve Mode**: Places orders one tick better than the current best bid/ask if there's room
///     - Attempts to gain queue priority by improving the market by exactly one tick
///     - Will not cross the spread (checks opposite side before improving)
///
/// - This algorithm will behave differently in PAPER_TRADING than on the real exchange, because
///   PAPER_TRADING does not affect the order book
///
/// The algorithm continuously monitors the market and repositions the quote as needed to maintain
/// competitiveness while respecting the specified constraints.
///
/// # Use Cases
/// - **Market Making**: Providing liquidity on one side of the market
/// - **Spread Trading**: Working the passive leg of a spread trade
/// - **Passive Execution**: Getting filled at favorable prices without crossing the spread
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSide;

impl Algo for QuoteOneSide {
    const NAME: &'static str = "QUOTE_ONE_SIDE";

    type Params = QuoteOneSideParams;
    type Status = QuoteOneSideStatus;
}

/// Whether to improve the market or join at the current best price
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ImproveOrJoin {
    /// Improve the market by one tick when possible
    Improve,
    /// Join at the current best price
    Join,
}

/// Parameters for the QuoteOneSide algorithm
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSideParams {
    pub symbol: String,
    pub marketdata_venue: MarketdataVenue,
    pub execution_venue: ExecutionVenue,
    pub account: AccountIdOrName,
    pub dir: Dir,
    pub quantity: Decimal,
    /// the most aggressive price the algo will quote at
    pub limit_price: Decimal,
    /// Maximum number of ticks less aggressive than the BBO to quote
    /// - `None`: No constraint on distance from BBO - will quote at any valid price up to the limit price
    /// - `Some(n)`: Will only quote if within n ticks of the best same-side price (BBO)
    ///   Orders beyond this distance are cancelled as they're unlikely to fill
    /// - Example: With `Some(5)` for a buy order, if best bid is 100, will only quote between 95-100
    pub max_ticks_outside: Option<Decimal>,
    /// Whether to improve the market or join at the current best price
    pub improve_or_join: ImproveOrJoin,
    /// Insert as 0, used for tracking fill quantity when modifying quote
    pub quantity_filled: Decimal,
    /// when being called from another algo
    pub parent_id: Option<OrderId>,
}

impl DisplaySymbols for QuoteOneSideParams {
    fn display_symbols(&self) -> Option<Vec<String>> {
        Some(vec![self.symbol.clone()])
    }
}

impl Validate for QuoteOneSideParams {
    fn validate(&self) -> Result<()> {
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if let Some(max_ticks) = self.max_ticks_outside {
            if !max_ticks.is_sign_positive() && max_ticks != Decimal::ZERO {
                bail!("max_ticks_outside must be non-negative");
            }
            if !max_ticks.is_integer() {
                bail!("max_ticks_outside must be an integer");
            }
        }
        Ok(())
    }
}

/// Current status of the QuoteOneSide algorithm
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuoteOneSideStatus {
    pub realized_avg_price: Option<Decimal>,
    pub quantity_filled: Decimal,
    pub current_quote_price: Option<Decimal>,
    /// Indicates whether the current quote is at the front of the queue (best price on our side)
    /// - For Buy orders: `true` when our quote price > previous best bid on the market
    /// - For Sell orders: `true` when our quote price < previous best ask on the market
    /// - Also `true` when we're the first to establish a quote on our side (no existing bid/ask)
    /// - This status updates dynamically as market conditions change and other orders arrive/cancel
    /// - Being front of queue provides priority for fills
    pub front_of_queue: bool,
    pub orders_sent: u32,
    /// Indicates whether the algorithm is currently cancelling an order
    pub is_cancelling: bool,
}
