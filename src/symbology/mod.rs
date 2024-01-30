/* Copyright 2023 Architect Financial Technologies LLC. This is free
 * software released under the GNU Affero Public License version 3. */

//! This is the protocol for sending symbology over the wire between
//! the symbology server and clients, and from the loaders to the
//! symbology server.

use crate::Str;
use anyhow::Result;
use bytes::Bytes;
use derive::FromValue;
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub mod cficode;
pub mod cpty;
pub mod market;
pub mod product;
pub mod query;
pub mod route;
pub mod venue;

pub use cpty::CptyId;
pub use market::{
    ExchangeMarketKind, Market, MarketId, MarketInfo, MarketKind, PoolMarketKind,
};
pub use product::{Product, ProductId, ProductKind};
pub use route::{Route, RouteId};
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

/// Symbology server/client wire type
#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub struct SymbologyUpdate {
    pub sequence_number: u64,
    pub kind: SymbologyUpdateKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub enum SymbologyUpdateKind {
    AddRoute(Route),
    RemoveRoute(RouteId),
    AddVenue(Venue),
    RemoveVenue(VenueId),
    AddProduct(Product),
    RemoveProduct(ProductId),
    AddMarket(Market),
    RemoveMarket(MarketId),
    /// compressed is a zstd compressed packed Pooled<Vec<SymbologyUpdateKind>>
    Snapshot {
        original_length: usize,
        compressed: Bytes,
    },
    /// elided version of [Snapshot] for no-op squashes--never stored in history,
    /// only used for synced clients
    SnapshotUnchanged(Bytes),
    #[pack(other)]
    Unknown,
}
