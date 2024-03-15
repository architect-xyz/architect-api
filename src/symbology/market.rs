//! Markets represent specific trading pairs on a given venue, via a given route.
//! For example, the market "BTC Crypto/USD*COINBASE/DIRECT" represents the direct
//! market connection to Coinbase's BTC/USD market.

use super::{Product, ProductId, Route, RouteId, Symbolic, Venue, VenueId};
use crate::{cpty, marketdata, uuid_val, Str};
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
            MarketKind::Exchange(ExchangeMarketKind { base: base.id, quote: quote.id }),
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
            MarketKind::Pool(PoolMarketKind { products: pool }),
            venue,
            route,
            exchange_symbol,
            extra_info,
        )
    }
}

impl Symbolic for Market {
    type Id = MarketId;

    fn type_name() -> &'static str {
        "market"
    }

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
    Exchange(ExchangeMarketKind),
    /// An unordered pool of products, e.g. a Uniswap pool or Curve 3-pool
    /// The type is still ordered for canonical naming purpose
    Pool(PoolMarketKind),
    #[pack(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct ExchangeMarketKind {
    pub base: ProductId,
    pub quote: ProductId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct PoolMarketKind {
    pub products: SmallVec<[ProductId; 2]>,
}

#[cfg_attr(feature = "juniper", juniper::graphql_object)]
impl PoolMarketKind {
    pub fn products(&self) -> Vec<ProductId> {
        self.products.iter().copied().collect()
    }
}

/// Cpty-specific info about a market
#[derive(Debug, Display, Clone, Serialize, Deserialize, Pack)]
#[enum_dispatch(NormalizedMarketInfo)]
#[serde(tag = "type", content = "value")]
#[rustfmt::skip]
pub enum MarketInfo {
    #[pack(tag(  0))] Test(TestMarketInfo),
    #[pack(tag(106))] B2C2(cpty::b2c2::B2C2MarketInfo),
    #[pack(tag(114))] Binance(cpty::binance::BinanceMarketInfo),
    #[pack(tag(112))] CboeDigital(cpty::cboe_digital::CboeDigitalMarketInfo),
    #[pack(tag(100))] Coinbase(cpty::coinbase::CoinbaseMarketInfo),
    #[pack(tag(111))] CoinbasePrime(cpty::coinbase_prime::CoinbasePrimeMarketInfo),
    #[pack(tag(104))] Cqg(cpty::cqg::CqgMarketInfo),
    #[pack(tag(113))] Cumberland(cpty::cumberland::CumberlandMarketInfo),
    #[pack(tag(110))] DYDX(cpty::dydx::DYDXMarketInfo),
    #[pack(tag(105))] Databento(marketdata::databento::DatabentoMarketInfo),
    #[pack(tag(115))] Deltix(cpty::deltix::DeltixMarketInfo),
    #[pack(tag(101))] Deribit(cpty::deribit::DeribitMarketInfo),
    #[pack(tag(109))] FalconX(cpty::falconx::FalconXMarketInfo),
    #[pack(tag(108))] Galaxy(cpty::galaxy::GalaxyMarketInfo),
    #[pack(tag(102))] Kraken(cpty::kraken::KrakenMarketInfo),
    #[pack(tag(103))] Okx(cpty::okx::OkxMarketInfo),
    #[pack(tag(107))] Wintermute(cpty::wintermute::WintermuteMarketInfo),
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

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct TestMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub is_delisted: bool,
}

impl std::fmt::Display for TestMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

impl NormalizedMarketInfo for TestMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        self.is_delisted
    }
}
