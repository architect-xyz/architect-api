//! Symbology netidx client/server protocol

#![cfg(feature = "netidx")]

use crate::symbology::{
    Market, MarketId, Product, ProductId, Route, RouteId, Venue, VenueId,
};
use bytes::Bytes;
use derive::FromValue;
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};

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
