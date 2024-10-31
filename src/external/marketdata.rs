use crate::{
    marketdata::{CandleV1, CandleWidth},
    symbology::MarketId,
    Dir,
};
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

    pub fn advance(&mut self) {
        self.sequence_number += 1;
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

impl std::fmt::Display for SequenceIdAndNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.sequence_id, self.sequence_number)
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
    /// Set of (price, level) updates. If zero, the price level
    /// has been removed from the book.
    #[serde(rename = "b")]
    pub bids: Vec<(Decimal, Decimal)>,
    /// Set of (price, level) updates. If zero, the price level
    /// has been removed from the book.
    #[serde(rename = "a")]
    pub asks: Vec<(Decimal, Decimal)>,
}

impl L2BookDiff {
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

/// To build a book from a stream of updates, the client should first subscribe to
/// this update stream, which then returns a stream starting with a snapshot and
/// following with diffs.
///
/// Diffs should be applied consecutively to the snapshot in order to reconstruct
/// the state of the book.
///
/// ```rust
/// # use architect_api::external::marketdata::*;
/// # use std::collections::BTreeMap;
/// # use rust_decimal::Decimal;
/// # use rust_decimal_macros::dec;
///
/// /// Suppose we receive this snapshot from the server:
/// let snapshot: L2BookUpdate = serde_json::from_str(r#"{
///     "t": "s",
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
/// book.insert(dec!(99.00), 3);
/// book.insert(dec!(98.78), 2);
/// book.insert(dec!(100.00), 1);
/// book.insert(dec!(100.10), 2);
///
/// /// Then we receive this update:
/// let diff: L2BookUpdate = serde_json::from_str(r#"{
///     "t": "d",
///     "ts": 1729700839,
///     "tn": 0,
///     "sid": 123,
///     "sn": 9000,
///     "b": [["99.00", "1"]],
///     "a": []
/// }"#)?;
///
/// /// Verify that the sequence number is correct
/// assert!(diff.sequence().is_next_in_sequence(&snapshot.sequence()));
///
/// /// Apply the update to our book
/// book.insert(dec!(99.00), 1);
///
/// // Suppose we then receive this update:
/// let diff: L2BookUpdate = serde_json::from_str(r#"{
///     "t": "d",
///     "ts": 1729700841,
///     "tn": 0,
///     "sid": 123,
///     "sn": 9005,
///     "b": [],
///     "a": [["103.00", "1"]]
/// }"#)?;
///
/// /// We shouldn't apply this update because it's not next in sequence!
/// assert_eq!(diff.sequence().is_next_in_sequence(&snapshot.sequence()), false);
///
/// /// Or if we had received this update:
/// let diff: L2BookUpdate = serde_json::from_str(r#"{
///     "t": "d",
///     "ts": 1729700841,
///     "tn": 0,
///     "sid": 170,
///     "sn": 9001,
///     "b": [],
///     "a": [["103.00", "1"]]
/// }"#)?;
///
/// /// It appears that the sequence id is changed, signalling a new sequence.
/// /// In this case, we should re-request the snapshot from the server.
/// assert_eq!(diff.sequence().is_next_in_sequence(&snapshot.sequence()), false);
///
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum L2BookUpdate {
    #[serde(rename = "s")]
    Snapshot(L2BookSnapshot),
    #[serde(rename = "d")]
    Diff(L2BookDiff),
}

impl L2BookUpdate {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Snapshot(snapshot) => snapshot.timestamp(),
            Self::Diff(diff) => diff.timestamp(),
        }
    }

    pub fn sequence(&self) -> SequenceIdAndNumber {
        match self {
            Self::Snapshot(snapshot) => snapshot.sequence,
            Self::Diff(diff) => diff.sequence,
        }
    }

    pub fn is_snapshot(&self) -> bool {
        match self {
            Self::Snapshot(_) => true,
            Self::Diff(_) => false,
        }
    }
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

// Subscribe to candles for a single market.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_candles",
    response = "Candle",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeCandlesRequest {
    pub market_id: MarketId,
    /// If None, subscribe from all candle widths on the feed
    pub candle_width: Option<Vec<CandleWidth>>,
}

// Subscribe to a single candle width across many markets.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_many_candles",
    response = "Candle",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeManyCandlesRequest {
    /// If None, subscribe from all symbols on the feed
    pub market_ids: Option<Vec<MarketId>>,
    pub candle_width: CandleWidth,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_trades",
    response = "Trade",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeTradesRequest {
    /// If None, subscribe from all symbols on the feed
    pub market_id: Option<MarketId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    #[serde(rename = "m")]
    pub market_id: MarketId,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(rename = "w")]
    pub width: CandleWidth,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "bv")]
    pub buy_volume: Decimal,
    #[serde(rename = "sv")]
    pub sell_volume: Decimal,
    #[serde(rename = "mo", skip_serializing_if = "Option::is_none")]
    pub mid_open: Option<Decimal>,
    #[serde(rename = "mc", skip_serializing_if = "Option::is_none")]
    pub mid_close: Option<Decimal>,
    #[serde(rename = "mh", skip_serializing_if = "Option::is_none")]
    pub mid_high: Option<Decimal>,
    #[serde(rename = "ml", skip_serializing_if = "Option::is_none")]
    pub mid_low: Option<Decimal>,
    #[serde(rename = "bo", skip_serializing_if = "Option::is_none")]
    pub bid_open: Option<Decimal>,
    #[serde(rename = "bc", skip_serializing_if = "Option::is_none")]
    pub bid_close: Option<Decimal>,
    #[serde(rename = "bh", skip_serializing_if = "Option::is_none")]
    pub bid_high: Option<Decimal>,
    #[serde(rename = "bl", skip_serializing_if = "Option::is_none")]
    pub bid_low: Option<Decimal>,
    #[serde(rename = "ao", skip_serializing_if = "Option::is_none")]
    pub ask_open: Option<Decimal>,
    #[serde(rename = "ac", skip_serializing_if = "Option::is_none")]
    pub ask_close: Option<Decimal>,
    #[serde(rename = "ah", skip_serializing_if = "Option::is_none")]
    pub ask_high: Option<Decimal>,
    #[serde(rename = "al", skip_serializing_if = "Option::is_none")]
    pub ask_low: Option<Decimal>,
}

impl Candle {
    pub fn from_candle_v1(
        market_id: MarketId,
        candle: CandleV1,
        width: CandleWidth,
    ) -> Self {
        Self {
            market_id,
            timestamp: candle.time.timestamp(),
            timestamp_ns: candle.time.timestamp_subsec_nanos(),
            width,
            open: candle.open,
            high: candle.high,
            low: candle.low,
            close: candle.close,
            volume: candle.volume,
            buy_volume: candle.buy_volume,
            sell_volume: candle.sell_volume,
            mid_open: candle.mid_open,
            mid_close: candle.mid_close,
            mid_high: candle.mid_high,
            mid_low: candle.mid_low,
            bid_open: candle.bid_open,
            bid_close: candle.bid_close,
            bid_high: candle.bid_high,
            bid_low: candle.bid_low,
            ask_open: candle.ask_open,
            ask_close: candle.ask_close,
            ask_high: candle.ask_high,
            ask_low: candle.ask_low,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "m")]
    pub market_id: MarketId,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(rename = "d")]
    pub direction: Option<Dir>, // maker dir
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub size: Decimal,
}

impl Trade {
    pub fn new(
        market_id: MarketId,
        price: Decimal,
        size: Decimal,
        direction: Option<Dir>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            market_id,
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            direction,
            price,
            size,
        }
    }
}
