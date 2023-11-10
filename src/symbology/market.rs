//! Markets represent specific trading pairs on a given venue, via a given route.
//! For example, the market "BTC Crypto/USD*COINBASE/DIRECT" represents the direct
//! market connection to Coinbase's BTC/USD market.

use super::{ProductId, RouteId, Symbolic, VenueId};
use crate::{cpty, uuid_val, Str};
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::{uuid, Uuid};

static MARKET_NS: Uuid = uuid!("0bfe858c-a749-43a9-a99e-6d1f31a760ad");
uuid_val!(MarketId, MARKET_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct Market {
    pub id: MarketId,
    pub name: Str,
    pub kind: MarketKind,
    pub venue: VenueId,
    pub route: RouteId,
    pub exchange_symbol: Str,
    pub extra_info: MarketInfo,
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
    Pool(SmallVec<[ProductId; 2]>),
    #[pack(other)]
    Unknown,
}

/// Cpty-specific info about a market
#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
#[serde(tag = "type", content = "value")]
pub enum MarketInfo {
    #[pack(tag(100))]
    Coinbase(cpty::coinbase::CoinbaseMarketInfo),
}

// TODO: CommonMarketInfo trait to extract some bs when you don't care spec
