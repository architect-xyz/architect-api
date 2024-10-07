/* Copyright 2023 Architect Financial Technologies LLC. This is free
 * software released under the GNU Affero Public License version 3. */

//! This is the protocol for sending symbology over the wire between
//! the symbology server and clients, and from the loaders to the
//! symbology server.

pub mod cficode;
pub mod cpty;
pub mod market;
pub mod product;
pub mod protocol;
pub mod query;
pub mod route;
pub mod venue;

use crate::Str;
use anyhow::Result;
pub use cpty::CptyId;
pub use market::{
    ExchangeMarketKind, Market, MarketId, MarketInfo, MarketKind, PoolMarketKind,
};
pub use product::{EventContracts, Product, ProductId, ProductKind};
#[cfg(feature = "netidx")]
pub use protocol::{SymbologyUpdate, SymbologyUpdateKind};
pub use route::{Route, RouteId};
use std::{fmt::Display, str::FromStr};
pub use venue::{Venue, VenueId};

/// All named symbology identifiers implement the trait Symbolic, which specifies
/// some common minimum functionality.
pub trait Symbolic: Clone + 'static {
    type Id: Copy + Ord + Eq + FromStr + From<Str> + Display;

    fn type_name() -> &'static str;
    fn id(&self) -> Self::Id;
    fn name(&self) -> Str;
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
