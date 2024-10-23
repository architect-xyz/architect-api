use crate::{symbology::MarketId, utils::dir::DirAsCharUpper, Dir};
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshot", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshotRequest {
    pub market_id: MarketId,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshots", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshotsRequest {
    pub market_ids: Vec<MarketId>,
}

pub type L1BookSnapshots = Vec<L1BookSnapshot>;

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_l1_book_snapshots",
    response = "L1BookSnapshot",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeL1BookSnapshotsRequest {
    /// If None, subscribe from all symbols on the feed
    pub market_ids: Option<Vec<MarketId>>,
}

impl From<Vec<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_ids: Vec<MarketId>) -> Self {
        Self { market_ids: Some(market_ids) }
    }
}

impl From<Option<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_id: Option<MarketId>) -> Self {
        Self { market_ids: market_id.map(|id| vec![id]) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1BookSnapshot {
    #[serde(rename = "m")]
    pub market_id: MarketId,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none", default)]
    pub epoch: Option<i64>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none", default)]
    pub seqno: Option<u64>,
    #[serde(rename = "b")]
    pub best_bid: Option<(Decimal, Decimal)>,
    #[serde(rename = "a")]
    pub best_ask: Option<(Decimal, Decimal)>,
}

impl L1BookSnapshot {
    pub fn new(
        market_id: MarketId,
        timestamp: DateTime<Utc>,
        epoch: Option<DateTime<Utc>>,
        seqno: Option<u64>,
        best_bid: Option<(Decimal, Decimal)>,
        best_ask: Option<(Decimal, Decimal)>,
    ) -> Self {
        Self {
            market_id,
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            epoch: epoch.map(|e| e.timestamp()),
            seqno,
            best_bid,
            best_ask,
        }
    }

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }

    pub fn epoch(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.epoch?, 0)
    }
}

/// Sequence id for distinguishing runs of sequence numbers.
type SequenceId = u64;

/// Unique sequence id and number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceIdAndNumber {
    #[serde(rename = "sid")]
    pub sequence_id: SequenceId,
    #[serde(rename = "sn")]
    pub sequence_number: u64,
}

impl SequenceIdAndNumber {
    pub fn new(sequence_id: SequenceId, sequence_number: u64) -> Self {
        Self { sequence_id, sequence_number }
    }

    pub fn new_random() -> Self {
        Self::new(rand::random::<SequenceId>(), 0)
    }

    pub fn next(&self) -> Self {
        Self::new(self.sequence_id, self.sequence_number + 1)
    }

    pub fn is_next_in_sequence(&self, previous: &Self) -> bool {
        self.sequence_id == previous.sequence_id
            && self.sequence_number == previous.sequence_number + 1
    }
}

impl PartialOrd for SequenceIdAndNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.sequence_id != other.sequence_id {
            // sequence numbers are not from the same sequence--incomparable
            None
        } else {
            Some(self.sequence_number.cmp(&other.sequence_number))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2BookSnapshot {
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(rename = "b")]
    pub bids: Vec<(Decimal, Decimal)>,
    #[serde(rename = "a")]
    pub asks: Vec<(Decimal, Decimal)>,
}

impl L2BookSnapshot {
    pub fn new(
        timestamp: DateTime<Utc>,
        sequence: SequenceIdAndNumber,
        bids: Vec<(Decimal, Decimal)>,
        asks: Vec<(Decimal, Decimal)>,
    ) -> Self {
        Self {
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            sequence,
            bids,
            asks,
        }
    }

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2BookDiff {
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(rename = "s")]
    pub side: DirAsCharUpper,
    #[serde(rename = "p")]
    pub price: Decimal,
    /// If zero, the price level has been removed from the book
    #[serde(rename = "q")]
    pub quantity: Decimal,
}

impl L2BookDiff {
    pub fn new(
        timestamp: DateTime<Utc>,
        sequence: SequenceIdAndNumber,
        side: Dir,
        price: Decimal,
        quantity: Decimal,
    ) -> Self {
        Self {
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            sequence,
            side: side.into(),
            price,
            quantity,
        }
    }

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

/// To build a book from a stream of updates, the client should first subscribe to
/// this update stream, then request a snapshot from the same server. Book updates
/// should be applied consecutively to the snapshot in order to reconstruct the
/// state of the book.
///
/// ```rust
/// /// Suppose we receive this snapshot from the server:
/// let snapshot: L2BookSnapshot = serde_json::from_str(r#"{
///     "ts": 1729700837,
///     "tn": 0,
///     "sid": 123,
///     "sn": 8999,
///     "b": [["99.00", "3"], ["98.78", "2"]],
///     "a": [["100.00", "1"], ["100.10", "2"]]
/// }"#)?;
///
/// /// It corresponds to the following book:
/// let mut book = BTreeMap::new();
/// book.insert(99.00, 3);
/// book.insert(98.78, 2);
/// book.insert(100.00, 1);
/// book.insert(100.10, 2);
///
/// /// Then we receive this update:
/// let update: L2BookUpdate = serde_json::from_str(r#"{
///     "ts": 1729700839,
///     "tn": 0,
///     "sid": 123,
///     "sn": 9000,
///     "s": "B",
///     "p": "99.00",
///     "q": "1"
/// }"#)?;
///
/// /// Verify that the sequence number is correct
/// assert!(update.sequence.is_next_in_sequence(&snapshot.sequence));
///
/// /// Apply the update to our book
/// book.insert(99.00, 1);
///
/// // Suppose we then receive this update:
/// let update: L2BookUpdate = serde_json::from_str(r#"{
///     "ts": 1729700841,
///     "tn": 0,
///     "sid": 123,
///     "sn": 9005,
///     "s": "S",
///     "p": "103.00",
///     "q": "1"
/// }"#)?;
///
/// /// We shouldn't apply this update because it's not next in sequence!
/// assert_eq!(update.sequence.is_next_in_sequence(&snapshot.sequence), false);
///
/// /// Or if we had received this update:
/// let update: L2BookUpdate = serde_json::from_str(r#"{
///     "ts": 1729700841,
///     "tn": 0,
///     "sid": 170,
///     "sn": 9001,
///     "s": "S",
///     "p": "103.00",
///     "q": "1"
/// }"#)?;
///
/// /// It appears that the sequence id is changed, signalling a new sequence.
/// /// In this case, we should re-request the snapshot from the server.
/// assert_eq!(update.sequence.is_next_in_sequence(&snapshot.sequence), false);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum L2BookUpdate {
    #[serde(rename = "s")]
    Snapshot(L2BookSnapshot),
    #[serde(rename = "d")]
    Diff(L2BookDiff),
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l2_book_snapshot", response = "L2BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2BookSnapshotRequest {
    pub market_id: MarketId,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_l2_book_updates",
    response = "L2BookUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeL2BookUpdatesRequest {
    pub market_id: MarketId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalL2BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryExternalL2BookSnapshot {
    pub market_id: MarketId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3BookSnapshot {
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(rename = "b")]
    pub bids: Vec<(u64, Decimal, Decimal)>,
    #[serde(rename = "a")]
    pub asks: Vec<(u64, Decimal, Decimal)>,
}
