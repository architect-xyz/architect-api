use chrono::{DateTime, Utc};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use std::str::FromStr;

// common, basic types which should cover a lot of use cases

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack)]
pub enum Dir {
    Buy,
    Sell,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(Self::Buy),
            "SELL" => Ok(Self::Sell),
            _ => Err(anyhow::anyhow!("invalid format: {s}")),
        }
    }
}

// common orderflow types suitable for generic oms/risk engine work
// maybe move this to orderflow.rs

#[derive(Debug, Clone, Pack)]
pub enum OrderflowMessage {
    Order(Order),
    Reject(Reject),
    Ack(Ack),
    Fill(Fill),
    Out(Out),
}

packed_value!(OrderflowMessage);

// TODO: cleaner if new() fns are impled, as well as choice accessors like id()
#[derive(Debug, Clone, Pack)]
pub struct Order {
    pub id: u64,
    pub target: String,
    pub dir: Dir,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Pack)]
pub struct Fill {
    pub order_id: u64,
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Pack)]
pub struct Reject {
    pub id: u64,
}

#[derive(Debug, Clone, Pack)]
pub struct Ack {
    pub id: u64,
}

#[derive(Debug, Clone, Pack)]
pub struct Out {
    pub id: u64,
}

pub mod cpty;
mod utils;
