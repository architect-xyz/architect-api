//! Markets represent specific trading pairs on a given venue, via a given route.
//! For example, the market "BTC Crypto/USD*COINBASE/DIRECT" represents the direct
//! market connection to Coinbase's BTC/USD market.

use super::{Product, ProductId, Route, RouteId, Symbolic, Venue, VenueId};
use crate::{cpty, uuid_val, Str};
use anyhow::Result;
use derive::FromValue;
use derive_more::Display;
use enum_dispatch::enum_dispatch;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::{uuid, Uuid};

static MARKET_NS: Uuid = uuid!("0bfe858c-a749-43a9-a99e-6d1f31a760ad");
uuid_val!(MarketId, MARKET_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub struct Market {
    pub id: MarketId,
    pub name: Str,
    pub kind: MarketKind,
    pub venue: VenueId,
    pub route: RouteId,
    pub exchange_symbol: Str,
    pub extra_info: MarketInfo,
}

impl Market {
    fn new(
        kind_name: &str,
        kind: MarketKind,
        venue: &Venue,
        route: &Route,
        exchange_symbol: &str,
        extra_info: MarketInfo,
    ) -> Result<Self> {
        let name = format!("{kind_name}*{}/{}", venue.name, route.name);
        Ok(Self {
            id: MarketId::from(&name),
            name: Str::try_from(name.as_str())?,
            kind,
            venue: venue.id,
            route: route.id,
            exchange_symbol: Str::try_from(exchange_symbol)?,
            extra_info,
        })
    }

    pub fn exchange(
        base: &Product,
        quote: &Product,
        venue: &Venue,
        route: &Route,
        exchange_symbol: &str,
        extra_info: MarketInfo,
    ) -> Result<Self> {
        Self::new(
            &format!("{}/{}", base.name, quote.name),
            MarketKind::Exchange { base: base.id, quote: quote.id },
            venue,
            route,
            exchange_symbol,
            extra_info,
        )
    }

    pub fn pool(
        products: impl Iterator<Item = Product>,
        venue: &Venue,
        route: &Route,
        exchange_symbol: &str,
        extra_info: MarketInfo,
    ) -> Result<Self> {
        let mut pool: SmallVec<[ProductId; 2]> = SmallVec::new();
        let mut kind_name = String::new();
        for p in products {
            kind_name.push_str(p.name.as_str());
            kind_name.push('/');
            pool.push(p.id);
        }
        Self::new(
            &kind_name,
            MarketKind::Pool(pool),
            venue,
            route,
            exchange_symbol,
            extra_info,
        )
    }
}

impl Symbolic for Market {
    type Id = MarketId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Str {
        self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
#[serde(tag = "type", content = "value")]
pub enum MarketKind {
    /// A regular exchange trading pair, e.g. Coinbase BTC/USD
    Exchange { base: ProductId, quote: ProductId },
    /// An unordered pool of products, e.g. a Uniswap pool or Curve 3-pool
    /// The type is still ordered for canonical naming purpose
    Pool(SmallVec<[ProductId; 2]>),
    #[pack(other)]
    Unknown,
}

/// Cpty-specific info about a market
#[derive(Debug, Display, Clone, Serialize, Deserialize, Pack)]
#[enum_dispatch(NormalizedMarketInfo)]
#[serde(tag = "type", content = "value")]
#[rustfmt::skip]
pub enum MarketInfo {
    #[pack(tag(100))] Coinbase(cpty::coinbase::CoinbaseMarketInfo),
    #[pack(tag(104))] Cqg(cpty::cqg::CqgMarketInfo),
    #[pack(tag(101))] Deribit(cpty::deribit::DeribitMarketInfo),
    #[pack(tag(102))] Kraken(cpty::kraken::KrakenMarketInfo),
    #[pack(tag(103))] Okx(cpty::okx::OkxMarketInfo),
}

#[enum_dispatch]
pub trait NormalizedMarketInfo {
    /// Return the tick size of the market or 1 if unknown
    fn tick_size(&self) -> Decimal;

    /// Return the step size of the market
    fn step_size(&self) -> Decimal;

    /// Return if the market is delisted
    fn is_delisted(&self) -> bool;
}
