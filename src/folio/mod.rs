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
use std::{collections::BTreeMap, sync::Arc};
use uuid::Uuid;

pub static SCHEMA: &'static str = include_str!("schema.sql");

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
    GetAllAccountSummaries,
    AllAccountSummaries(Vec<(CptyId, Arc<AccountSummaries>)>),
    /// Request to cpty for balances snapshot
    GetAccountSummaries(CptyId),
    /// Response from cpty with balances snapshot
    AccountSummaries(CptyId, Option<Arc<AccountSummaries>>),
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

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, Default)]
pub struct AccountSummary {
    pub balances: BTreeMap<ProductId, Balance>,
    // There could be multiple open positions for a particular Market,
    // so this is not a BTreeMap like [balances].
    pub positions: Vec<Position>,
    pub profit_loss: Option<Decimal>,
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
            FolioMessage::GetFills(id, ..) | FolioMessage::GetSyncStatus(id, ..) => {
                Some(*id)
            }
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            FolioMessage::Fills(id, ..) => *id,
            FolioMessage::GetSyncStatusResponse(id, ..) => Some(*id),
            _ => None,
        }
    }
}
