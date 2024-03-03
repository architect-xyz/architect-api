use crate::{
    orderflow::{AberrantFill, Fill},
    symbology::*,
    HalfOpenRange,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

pub static SCHEMA: &'static str = include_str!("schema.sql");

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum FolioMessage {
    GetFillsQuery(MarketFilter, HalfOpenRange<Option<DateTime<Utc>>>),
    GetFillsQueryResponse(
        MarketFilter, 
        HalfOpenRange<Option<DateTime<Utc>>>,
        Arc<Vec<Result<Fill, AberrantFill>>>,
    ),
    GetFills(
        CptyId,
        HalfOpenRange<Option<DateTime<Utc>>>,
    ),
    Fills(
        CptyId,
        HalfOpenRange<Option<DateTime<Utc>>>,
        Arc<Vec<Result<Fill, AberrantFill>>>,
    ),
    RealtimeFill(Result<Fill, AberrantFill>),
    GetAllBalances,
    GetBalances(CptyId),
    AllBalances(Vec<(CptyId, Arc<BTreeMap<ProductId, Decimal>>)>),
    Balances(CptyId, Arc<BTreeMap<ProductId, Decimal>>),
    /// Control message to folio to update balances
    UpdateBalances,
    /// Control messages to folio to sync fills
    SyncFillsForward,
    SyncFillsBackward(CptyId),
    InvalidateSyncBefore(CptyId, DateTime<Utc>),
    InvalidateSyncAfter(CptyId, DateTime<Utc>),
}

#[derive(Copy, Debug, Clone, Pack, FromValue, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketFilter {
    pub venue: Option<VenueId>,
    pub route: Option<RouteId>,
    pub base: Option<ProductId>,
    pub quote: Option<ProductId>,
}
