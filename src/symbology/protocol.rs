//! Symbology netidx client/server protocol

use crate::symbology::{
    Market, MarketId, Product, ProductId, Route, RouteId, Venue, VenueId,
};
use bytes::Bytes;
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};

/// Symbology server/client wire type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct SymbologyUpdate {
    pub sequence_number: u64,
    pub kind: SymbologyUpdateKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
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
    #[cfg_attr(feature = "netidx", pack(other))]
    Unknown,
}
