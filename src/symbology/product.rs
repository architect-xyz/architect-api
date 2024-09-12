//! Product are specific assets, liabilities, tokens, etc. things that can be owned,
//! traded, or exchanged.

use super::{Symbolic, VenueId};
use crate::{uuid_val, Str};
use anyhow::Result;
use bytes::Bytes;
use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::{uuid, Uuid};

static PRODUCT_NS: Uuid = uuid!("bb25a7a7-a61c-485a-ac29-1de369a6a043");
uuid_val!(ProductId, PRODUCT_NS);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct Product {
    pub id: ProductId,
    pub name: Str,
    pub kind: ProductKind,
}

impl Product {
    pub fn new(name: &str, kind: ProductKind) -> Result<Product> {
        Ok(Product { id: ProductId::from(name), name: Str::try_from(name)?, kind })
    }
}

impl Symbolic for Product {
    type Id = ProductId;

    fn type_name() -> &'static str {
        "product"
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Str {
        self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[serde(tag = "type", content = "value")]
pub enum InstrumentType {
    Inverse,
    Linear,
    Quanto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[serde(tag = "type", content = "value")]
pub enum ProductKind {
    // CR alee: deprecate in favor of Coin, Token without params
    Coin {
        token_info: BTreeMap<VenueId, TokenInfo>,
    },
    Fiat,
    Equity,
    Perpetual {
        underlying: Option<ProductId>,
        multiplier: Option<Decimal>,
        instrument_type: Option<InstrumentType>,
    },
    /// The one guarantee for [underlying] if set is that it can
    /// be used to uniquely identify strips of related futures
    Future {
        underlying: Option<ProductId>,
        multiplier: Option<Decimal>,
        expiration: Option<DateTime<Utc>>,
        instrument_type: Option<InstrumentType>,
    },
    FutureSpread {
        same_side_leg: Option<ProductId>,
        opp_side_leg: Option<ProductId>,
    },
    /// The one guarantee for [underlying] if set is that it can
    /// be used to uniquely identify strips of related options
    Option {
        underlying: Option<ProductId>,
        multiplier: Option<Decimal>,
        expiration: Option<DateTime<Utc>>,
        instrument_type: Option<InstrumentType>,
    },
    Index,
    Commodity,
    /// Event contracts are products akin to binary options
    /// which settle to an outcome of a future event.
    ///
    /// Specific tradable event contracts are represented by
    /// the EventContract variant, e.g. FED-2024-SEP-CUT-25-YES
    /// and/or FED-2024-SEP-CUT-25-NO. for the YES
    /// and NO contracts of the "Fed to cut 25 bps" outcome.
    /// EventContract's are grouped into EventOutcome's,
    /// which pair the YES and NO contracts of an outcome
    /// together.  There are venues like KALSHI which have
    /// only one YES contract per outcome (the NO contract
    /// is implicit via short-selling the YES contract).
    ///
    /// EventOutcomes are grouped into Events, e.g.
    /// FED-2024-SEP is an Event with the following mutually
    /// exclusive outcomes:
    ///
    /// - FED-2024-SEP-HIKE
    /// - FED-2024-SEP-CUT-0
    /// - FED-2024-SEP-CUT-25
    /// - FED-2024-SEP-CUT-ABOVE-25
    ///
    /// Events _may_ be grouped into EventSeries, e.g. all
    /// FED events belong to the same series of events.
    ///
    /// The grouping of EventContracts into outcomes,
    /// events, and event series are indicative and mostly
    /// for display purposes, and don't necessarily imply
    /// anything about the settlement of individual
    /// event contracts.
    EventSeries {
        display_name: String,
    },
    Event {
        series: Option<ProductId>,
        outcomes: Vec<ProductId>,
        mutually_exclusive: Option<bool>,
        expiration: Option<DateTime<Utc>>,
    },
    EventOutcome {
        display_order: Option<u32>,
        contracts: EventContracts,
        display_name: String,
    },
    EventContract {
        expiration: Option<DateTime<Utc>>,
    },
    #[cfg_attr(feature = "netidx", pack(other))]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub enum EventContracts {
    Single { yes: ProductId, yes_alias: Option<Str> },
    Dual { yes: ProductId, yes_alias: Option<Str>, no: ProductId, no_alias: Option<Str> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLUnion))]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub enum TokenInfo {
    ERC20(ERC20TokenInfo),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct ERC20TokenInfo {
    // CR alee: don't use bytes, just use the packed ethers type
    pub address: Bytes,
    pub decimals: u8,
}

#[cfg(feature = "juniper")]
#[cfg_attr(feature = "juniper", juniper::graphql_object)]
impl ERC20TokenInfo {
    // CR alee: resolve above CR before implementing address()

    pub fn decimals(&self) -> crate::utils::graphql_scalars::U8 {
        self.decimals.into()
    }
}
