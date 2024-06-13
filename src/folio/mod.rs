#![cfg(feature = "netidx")]

use crate::{
    orderflow::{AberrantFill, Fill},
    symbology::*,
    utils::{half_open_range::ClampSign, messaging::MaybeRequest},
    AccountId, Dir, HalfOpenRange,
};
use chrono::{DateTime, NaiveDate, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use uuid::Uuid;

pub static SCHEMA: &'static str = include_str!("schema.sql");

/// Cpty should implement the following RPCs/messages for Folio integration:
///
/// - GetFills/Fills
/// - GetAccountSummaries/AccountSummaries
/// - Fills (realtime unsolicited fill dropcopy)
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum FolioMessage {
    GetFillsQuery(MarketFilter, HalfOpenRange<Option<DateTime<Utc>>>),
    GetFillsQueryResponse(
        MarketFilter,
        HalfOpenRange<Option<DateTime<Utc>>>,
        Arc<Vec<Result<Fill, AberrantFill>>>,
    ),
    GetFills(Uuid, GetFills),
    Fills(Option<Uuid>, CptyId, Result<Fills, GetFillsError>), // None for unsolicited
    /// Cptys should dropcopy realtime fills to Folio as they become known
    RealtimeFill(Result<Fill, AberrantFill>),
    /// Request account summaries snapshot from all cptys, grouped by cpty
    ///
    /// - request id
    /// - account ids (None for all accounts)
    GetAllAccountSummaries(Uuid, Option<Arc<BTreeSet<AccountId>>>),
    AllAccountSummaries(Uuid, Vec<(CptyId, Arc<AccountSummaries>)>),
    /// Request account summaries snapshot; can be called internally
    /// (as Folio <-> Cpty), or externally (client <-> Folio)
    ///
    /// - request id
    /// - cpty id
    /// - account ids (None for all accounts)
    GetAccountSummaries(Uuid, CptyId, Option<Arc<BTreeSet<AccountId>>>),
    /// Response from cpty with balances snapshot;
    /// may be unsolicited (response_id = None) from cptys
    AccountSummaries(Option<Uuid>, CptyId, Option<Arc<AccountSummaries>>),
    /// Control message to folio to update balances
    UpdateAccountSummaries,
    /// Control messages to folio to sync fills
    SyncFillsForward,
    SyncFillsBackward(CptyId),
    InvalidateSyncBefore(CptyId, DateTime<Utc>),
    InvalidateSyncAfter(CptyId, DateTime<Utc>),
    /// Account advertising
    AdvertiseAccounts(CptyId, Arc<Vec<AccountId>>),
    GetSyncStatus(Uuid, CptyId),
    GetSyncStatusResponse(Uuid, FolioSyncStatus),
    /// Take a snapshot of balances and upsert
    SnapshotBalances,
}

#[derive(Copy, Debug, Clone, Pack, FromValue, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketFilter {
    pub venue: Option<VenueId>,
    pub route: Option<RouteId>,
    pub base: Option<ProductId>,
    pub quote: Option<ProductId>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct AccountSummaries {
    pub snapshot_ts: DateTime<Utc>,
    pub by_account: BTreeMap<AccountId, AccountSummary>,
}

impl AccountSummaries {
    pub fn filter_accounts(&self, accounts: &BTreeSet<AccountId>) -> Self {
        let by_account = self
            .by_account
            .iter()
            .filter_map(|(account_id, summary)| {
                if accounts.contains(account_id) {
                    Some((*account_id, summary.clone()))
                } else {
                    None
                }
            })
            .collect();
        Self { snapshot_ts: self.snapshot_ts, by_account }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, Default)]
pub struct AccountSummary {
    pub balances: BTreeMap<ProductId, Balance>,
    // There could be multiple open positions for a particular Market,
    // so this is not a BTreeMap like [balances].
    pub positions: Vec<Position>,
    pub profit_loss: Option<Decimal>,
    pub clearing_venue: Option<VenueId>,
}

impl AccountSummary {
    pub fn from_simple_balances(balances: BTreeMap<ProductId, Decimal>) -> Self {
        Self {
            balances: balances
                .into_iter()
                .map(|(product_id, total)| {
                    (product_id, Balance { total: Some(total), ..Default::default() })
                })
                .collect(),
            positions: vec![],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, Default)]
pub struct Balance {
    /// The total amount of this asset held in the account by the
    /// venue/broker.
    pub total: Option<Decimal>,

    /// Margin requirement calculated for worst-case based on open positions and working orders.
    pub total_margin: Option<Decimal>,

    /// Margin requirement calculated for worst-case based on open positions.
    pub position_margin: Option<Decimal>,

    /// Available account funds including balance, realized profit (or loss), collateral and credits.
    pub purchasing_power: Option<Decimal>,

    /// Cash available in the account beyond the required margin.
    pub cash_excess: Option<Decimal>,

    /// Cash balance from the last statement.
    pub yesterday_balance: Option<Decimal>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct Position {
    pub market_id: MarketId,

    pub quantity: Option<Decimal>,

    /// Average price used to open position
    pub average_price: Option<Decimal>,

    pub trade_time: Option<DateTime<Utc>>,

    /// The trade date according to the exchange
    /// settlement calendar.
    pub trade_date: Option<NaiveDate>,

    pub dir: Dir,

    pub break_even_price: Option<Decimal>,

    pub liquidation_price: Option<Decimal>,
}

/// Request to cpty for a certain range of fills.
///
/// Cpty is allowed to reply with a range smaller than requested,
/// for whatever reason (don't want to paginate, API limit)...
/// the clamp_sign tells you which side of the range should be
/// moved when shirking the range.
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct GetFills {
    pub cpty: CptyId,
    pub range: HalfOpenRange<Option<DateTime<Utc>>>,
    pub clamp_sign: ClampSign,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum GetFillsError {
    #[serde(other)]
    #[pack(other)]
    Unknown,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct Fills {
    pub range: HalfOpenRange<Option<DateTime<Utc>>>,
    pub fills: Arc<Vec<Result<Fill, AberrantFill>>>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct FolioSyncStatus {
    pub accounts_advertised: Arc<Vec<AccountId>>,
    pub synced_range: Option<HalfOpenRange<DateTime<Utc>>>,
    pub beginning_of_time: DateTime<Utc>,
    pub forward_sync_backoff: Option<DateTime<Utc>>,
    pub backfill_backoff: Option<DateTime<Utc>>,
}

impl MaybeRequest for FolioMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            FolioMessage::GetFills(id, ..)
            | FolioMessage::GetSyncStatus(id, ..)
            | FolioMessage::GetAccountSummaries(id, ..)
            | FolioMessage::GetAllAccountSummaries(id, ..) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            FolioMessage::Fills(id, ..) | FolioMessage::AccountSummaries(id, ..) => *id,
            FolioMessage::GetSyncStatusResponse(id, ..)
            | FolioMessage::AllAccountSummaries(id, ..) => Some(*id),
            _ => None,
        }
    }
}
