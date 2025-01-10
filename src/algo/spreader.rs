use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage, symbology::MarketId, Dir,
    HumanDuration, OrderId,
};
use anyhow::{bail, Result};
use quote_one_side::ImproveOrJoin;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub type SpreaderAlgoMessage = AlgoContainerMessage<
    SpreaderAlgoOrder,
    NoModification,
    AlgoPreview,
    SpreaderAlgoStatus,
    AlgoLog,
>;

/// An advanced algo for trading spreads between products in different order books.
///
/// Specifically, this algo is meant for trading spreads that there is not an exchange listed spread for
/// (e.g. a spread between two products that are not listed on the same exchange would be such a candidate).
///
/// The spreader will have legs with taking and quoting parameters.
/// Every leg will have at most one taking order out, one quoting order, and one quote hedge out at once respectively.
///
/// The taking params will be relative to the far side (ie the crossing side) of the bbo.
/// This will not necessarily cross (e.g. you send order one level away from far side so that others can aggress into you) completely but generally will cross.
/// The taking uses the algo take_and_chase.
///
/// The quoting params will be relative to the near side.
/// The quoting uses the algo quote_one_side.
///
/// There will be 2 execution types (assume N legs of the spread):
///     SIMULTANEOUS TAKE: all take orders, where all N legs fire at once
///         - there will be no orders out until the spread hits to desired level
///         - the algo will then send orders relative to the far side of the bbo
///         - the spread value is backed out using the potential fill prices relative to the far side of the bbo
///     QUOTE THEN TAKE: one leg is quoting, while the other legs take only after the quote fill (so there will be potentially N combos of this, one quote for each leg)
///         - the price of the quote is based on the required price to get the desired spread value, assuming that all the other legs are takes
///         - one product is quoting and the others will fire after
///         - for example, for SUM SpreadKind, the quote price is backed out from the desired spread price - SUM(other leg potential fill prices relative to the far side of the bbo)
///
/// The Preview of the algo will be the one where you can see the synthetic order book of the spread (which will be generated in a greedy manner over the leg order books)
///
/// Key features:
/// - SpreadKind: the way the spread is calculated
///     - Sum: this just sums px_i * ratio_i, example spreads would be ES-SPY, HO-G, BTC Fut - BTC Perp, BTC Perp - BTC Spot
///     - NetChangeInTicks: based on individual legs' net change from previous settle price, example spread would be NOB spread, 10s (ZN) vs 30s (ZB) spread
///     - Yield: for treasury futures, this is calculated based on yield
///     - Custom { formula: String }: based on the formula must watch out for division by zero, example spread would be X - Y / Z , where Y is a contract priced in another currency, and Z is the currency conversion rate
/// currently, only Sum has been implemented
///
/// - When the spread value is breached for TAKING (ie when buying spread value is LOWER than price, when selling spread value is HIGHER than price)
///     then the algo will fire on each leg for the spread_quantity * the ratio of the leg
///
/// For each leg, there will be a market, a quantity_ratio, a price_multiplier, and a price_offset.
/// The quantity_ratio is the ratio of the quantity of the leg to the total quantity of the spread.
/// The price_multiplier is the multiplier of the price of the leg to the total price of the spread.
/// Generally, the quantity_ratio and the price_multiplier will be the same,
/// The exceptions will generally be when a spread is traded in different units.
/// For example, Heating-Oil (HO) vs Gasoil (G):
///     - HO is priced in $ / per gallon, G is priced in $ per metric ton
///     - HO contract size is 42,000 gallons, G contract size is 100 metric tons
///     - 1 gallon of heating oil is around 0.00318 metric tons, so the price_multiplier would be 1 HO x -0.00318 G
///     - 42,000 gallons of heating oil is around 133.56 metric tons, so a quantity_ratio of 3 HO x -4 G would yield a similar tonnage (~400 vs 400 metric tons)
///
/// The price_offset is applied after the multiplier, and can be used to reflect costs.

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct SpreaderAlgoOrder {
    pub order_id: OrderId,
    pub trader: UserId,
    pub account: Option<AccountId>,
    pub spread_kind: SpreadKind,
    pub legs: Vec<Leg>, // must be length at least 2
    pub taking_parameters: Option<TakingParameters>,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
}

impl Into<AlgoOrder> for &SpreaderAlgoOrder {
    fn into(self) -> AlgoOrder {
        /*
        let markets = self
            .legs
            .iter()
            .flat_map(|leg| {
                leg.market.iter().map(|leg| leg.market_id).collect::<Vec<_>>()
            })
            .collect();
        */
        let markets = self.legs.iter().map(|leg| leg.market.market_id).collect();

        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: self.account,
            algo: AlgoKind::MarketMaker,
            parent_order_id: None,
            markets: Arc::new(markets),
        }
    }
}

impl SpreaderAlgoOrder {
    pub fn new(
        order_id: OrderId,
        trader: UserId,
        account: Option<AccountId>,
        spread_kind: SpreadKind,
        legs: Vec<Leg>,
        taking_parameters: Option<TakingParameters>,
        quantity: Decimal,
        price: Decimal,
        dir: Dir,
    ) -> Self {
        Self {
            order_id,
            trader,
            account,
            spread_kind,
            legs,
            taking_parameters,
            quantity,
            price,
            dir,
        }
    }
}

impl Validate for SpreaderAlgoOrder {
    fn validate(&self) -> Result<()> {
        if self.legs.len() < 2 {
            bail!("Must have at least 2 legs");
        }
        assert!(self.quantity > dec!(0), "quantity must be positive");

        match self.taking_parameters {
            Some(..) => {
                for leg in self.legs.iter() {
                    assert!(leg.leg_taking_parameters.is_some(), "Must have taking parameters for each leg if you have taking parameters");
                }
            }
            None => {
                assert!(self.legs.iter().any(|x| x.quoting_parameters.is_some()), "Must have quoting parameters for at least one leg if taking parameters is None");
            }
        }

        assert!(
            self.legs
                .iter()
                .any(|x| x.quoting_parameters.is_some()
                    || x.leg_taking_parameters.is_some()),
            "Each leg must either have leg_taking_parameters or quoting_parameters"
        );

        if let Some(take_parameters) = self.taking_parameters {
            take_parameters.validate()?;
        }

        for leg in &self.legs {
            leg.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct SpreaderAlgoStatus {
    pub algo_status: AlgoStatus,
    pub last_take_send_time: Option<DateTime<Utc>>,
    pub filled: Decimal,
    pub quantity: Decimal,
    pub take_state: Option<TakeState>,
}

#[derive(
    Debug, Clone, Pack, FromValue, Serialize, Deserialize, PartialEq, JsonSchema,
)]
pub enum TakeState {
    Waiting(String),
    Sent,
    Hanging { leg_counts: Vec<usize> },
    Done,
}

impl TryInto<AlgoStatus> for &SpreaderAlgoStatus {
    type Error = ();

    fn try_into(self) -> Result<AlgoStatus, ()> {
        Ok(self.algo_status)
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct Leg {
    pub market: LegMarket, // CR acho: This should be Vec<LegMarket> eventually
    pub leg_taking_parameters: Option<LegTakingParameters>,
    pub quoting_parameters: Option<QuotingParameters>,
}

impl Validate for Leg {
    fn validate(&self) -> Result<()> {
        self.market.validate()?;

        if let Some(leg_taking_parameters) = &self.leg_taking_parameters {
            leg_taking_parameters.validate()?;
        }

        if let Some(quoting_parameters) = &self.quoting_parameters {
            if let Some(max_ticks_outside) = quoting_parameters.max_ticks_outside {
                assert!(max_ticks_outside.is_sign_positive(),);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct LegMarket {
    pub market_id: MarketId,
    pub quantity_ratio: Decimal,
    pub price_multiplier: Decimal,
    pub price_offset: Decimal, // APPLIED AFTER MULTIPLIER, can be used to reflect costs
                               // consider making this directionless
}

impl Validate for LegMarket {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub enum SpreadKind {
    Sum, // this just sums px_i * ratio_i
    NetChangeInTicks,
    Yield,
    Custom { formula: String }, // must watch out for division by zero}
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum OverfillBehavior {
    CloseImmediately,
    HedgeAggressive,
    HedgePassive,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct QuotingParameters {
    pub max_ticks_outside: Option<Decimal>, // max ticks outside the market to quote
    pub improve_or_join: ImproveOrJoin,

    pub max_size: Option<Decimal>,
    pub lockout: Option<HumanDuration>,
    pub modify_quote_for_take: bool,
    // pub overfill_behavior: Option<OverfillBehavior>,
    // pub working_prices: u8, // how many prices to work at a time
    // pub take_using_quotes: bool, // this prevents double fill in the case of quoting two
    // pub cancellation_strategy: CancellationStrategy,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct LegTakingParameters {
    pub chase_levels: Decimal, // in number of ticks in the current leg to chase the hedge market

    pub dont_chase_if_opposite_side_qty_too_large: Option<Decimal>,
    pub start_maximally_aggressive: bool,
    pub chase_lockout: Option<HumanDuration>,

    pub take_hedge_offset: Option<Decimal>,
    // when firing a take in response to a quote fill from another leg,
    // number of ticks from bbo to be less aggressive by
    // in case of wide markets, you don't need to fully cross to execute
    // so positive would be less aggressive
    // negative would be more aggressive
}

impl Validate for LegTakingParameters {
    fn validate(&self) -> Result<()> {
        if let Some(dont_chase_if_opposite_side_qty_too_large) =
            self.dont_chase_if_opposite_side_qty_too_large
        {
            assert!(
                dont_chase_if_opposite_side_qty_too_large.is_sign_positive(),
                "dont_chase_if_opposite_side_qty_too_large must be positive"
            );
        }

        assert!(
            !self.chase_levels.is_sign_negative(),
            "chase_levels must be nonnegative"
        );
        assert!(self.chase_levels.is_integer(), "chase_levels must be an integer");

        assert!(
            self.take_hedge_offset.is_none()
                || self.take_hedge_offset.unwrap().is_sign_positive(),
            "take_hedge_offset must be positive"
        );

        assert!(
            self.take_hedge_offset.is_none()
                || self.take_hedge_offset.unwrap().is_integer(),
            "take_hedge_offset must be an integer"
        );

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct TakingParameters {
    // must also define LegTakingParameters for each leg
    pub min_quantity_threshold: Option<Decimal>,
    pub min_fire_quantity: Option<Decimal>,
    pub max_fire_quantity: Option<Decimal>,

    pub scale_with_largest_leg: bool,
    pub take_lockout: Option<HumanDuration>,
}
/*
if a spread meets spread price threshold (set via the price)
and the min_sweep_threshold, this will sweep at least the min_sweep_quantity
and post the remainder

The quantity sent will be at least the min_sweep_quantity
and will scale up by the available spread

as long as the spread price is breached, this will take *AT LEAST* the min fire_quantity level
any remainder of the clip that is not filled will remain in the orderbook
up to the max_fire_quantity depending on the quantity available

the scale_with_larger_leg flag will make the sweep quantity scale
with respect to the leg with the leg with largest spread quantity
will not post more than available on the largest side
will not fire unless the largest side of the spread has at least the leg_ratio * min_sweep_threshold amount
*/

impl Validate for TakingParameters {
    fn validate(&self) -> Result<()> {
        if let Some(min_quantity_threshold) = self.min_quantity_threshold {
            assert!(
                min_quantity_threshold > dec!(0),
                "min_quantity_threshold must be positive"
            );

            if let Some(min_fire_quantity) = self.min_fire_quantity {
                assert!(
                    min_fire_quantity >= min_quantity_threshold,
                    "min_fire_quantity must be greater than or equal to min_quantity_threshold"
                );
            }

            if let Some(max_fire_quantity) = self.max_fire_quantity {
                assert!(
                    max_fire_quantity >= min_quantity_threshold,
                    "max_fire_quantity must be greater than or equal to min_quantity_threshold"
                );
            }
        }

        if let Some(min_fire_quantity) = self.min_fire_quantity {
            assert!(min_fire_quantity > dec!(0), "min_fire_quantity must be positive");

            if let Some(max_fire_quantity) = self.max_fire_quantity {
                assert!(
                    max_fire_quantity >= min_fire_quantity,
                    "max_fire_quantity must be greater than or equal to min_fire_quantity"
                );
            }
        }

        if let Some(max_fire_quantity) = self.max_fire_quantity {
            assert!(max_fire_quantity > dec!(0), "max_fire_quantity must be positive");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub enum CancellationStrategy {
    FIFO,
    LIFO,
    WorstPrice,
    LowestPrio,
}
