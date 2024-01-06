use crate::{symbology::MarketId, Dir, OrderId, Str};
use anyhow::Result;
use chrono::{DateTime, Utc};
use enumflags2::{bitflags, BitFlags};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Order {
    pub id: OrderId,
    pub market: MarketId,
    pub dir: Dir,
    pub limit_price: Decimal,
    pub quantity: Decimal,
    pub account: Option<Str>,
    pub order_type: OrderType,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub expiration: Option<DateTime<Utc>>,
    pub post_only: bool,
}

impl Order {
    pub fn builder(
        id: OrderId,
        market: MarketId,
        dir: Dir,
        limit_price: Decimal,
        quantity: Decimal,
    ) -> OrderBuilder {
        OrderBuilder(Self {
            id,
            market,
            dir,
            limit_price,
            quantity,
            account: None,
            order_type: OrderType::Limit,
            trigger_price: None,
            time_in_force: TimeInForce::GoodTilDate,
            expiration: None,
            post_only: false,
        })
    }
}

pub struct OrderBuilder(Order);

impl OrderBuilder {
    pub fn account(mut self, account: Option<Str>) -> Self {
        self.0.account = account;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.0.order_type = order_type;
        self
    }

    pub fn trigger_price(mut self, trigger_price: Option<Decimal>) -> Self {
        self.0.trigger_price = trigger_price;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.0.time_in_force = time_in_force;
        self
    }

    pub fn expiration(mut self, expiration: Option<DateTime<Utc>>) -> Self {
        self.0.expiration = expiration;
        self
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.0.post_only = post_only;
        self
    }

    pub fn build(self) -> Result<Order> {
        // CR bharrison: add validation of invariants
        Ok(self.0)
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum OrderType {
    Limit,
    StopLossLimit,
    TakeProfitLimit,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum TimeInForce {
    GoodTilCancel,
    GoodTilDate,
    ImmediateOrCancel,
}

/// The state of an order
#[bitflags]
#[repr(u8)]
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Pack, Serialize, Deserialize, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum OrderStateFlags {
    Open,
    Rejected,
    Acked,
    Filled,
    Canceling,
    Canceled,
    Out,
    Stale, // we were expecting some state change but it was never confirmed
}

pub type OrderState = BitFlags<OrderStateFlags>;

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Cancel {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Reject {
    pub order_id: OrderId,
}

impl Reject {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Ack {
    pub order_id: OrderId,
}

impl Ack {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Out {
    pub order_id: OrderId,
}

impl Out {
    pub fn new(order_id: OrderId) -> Self {
        Self { order_id }
    }
}
