use crate::{
    marketdata::{CandleV1, CandleWidth},
    symbology::MarketId,
    utils::sequence::SequenceIdAndNumber,
    Dir,
};
use chrono::{DateTime, Utc};
use derive::grpc;
use derive_more::{Deref, DerefMut};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshot", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshotRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshots", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshotsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<MarketId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

pub type L1BookSnapshots = Vec<L1BookSnapshot>;

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_l1_book_snapshots",
    response = "L1BookSnapshot",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeL1BookSnapshotsRequest {
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<MarketId>>,
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

impl From<Vec<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_ids: Vec<MarketId>) -> Self {
        Self { market_ids: Some(market_ids), symbols: None }
    }
}

impl From<Option<MarketId>> for SubscribeL1BookSnapshotsRequest {
    fn from(market_id: Option<MarketId>) -> Self {
        Self { market_ids: market_id.map(|id| vec![id]), symbols: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshot {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    // CR alee: deprecated
    #[serde(rename = "e", skip_serializing_if = "Option::is_none", default)]
    #[schemars(title = "epoch")]
    pub epoch: Option<i64>,
    // CR alee: deprecated
    #[serde(rename = "n", skip_serializing_if = "Option::is_none", default)]
    #[schemars(title = "seqno")]
    pub seqno: Option<u64>,
    #[serde(rename = "b")]
    #[schemars(title = "best_bid")]
    pub best_bid: Option<(Decimal, Decimal)>,
    #[serde(rename = "a")]
    #[schemars(title = "best_ask")]
    pub best_ask: Option<(Decimal, Decimal)>,
}

impl L1BookSnapshot {
    pub fn new(
        market_id: Option<MarketId>,
        symbol: Option<String>,
        timestamp: DateTime<Utc>,
        epoch: Option<DateTime<Utc>>,
        seqno: Option<u64>,
        best_bid: Option<(Decimal, Decimal)>,
        best_ask: Option<(Decimal, Decimal)>,
    ) -> Self {
        Self {
            market_id,
            symbol,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L2BookSnapshot {
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(rename = "b")]
    #[schemars(title = "bids")]
    pub bids: Vec<(Decimal, Decimal)>,
    #[serde(rename = "a")]
    #[schemars(title = "asks")]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L2BookDiff {
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    /// Set of (price, level) updates. If zero, the price level
    /// has been removed from the book.
    #[serde(rename = "b")]
    #[schemars(title = "bids")]
    pub bids: Vec<(Decimal, Decimal)>,
    /// Set of (price, level) updates. If zero, the price level
    /// has been removed from the book.
    #[serde(rename = "a")]
    #[schemars(title = "asks")]
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
pub enum L2BookUpdate {
    #[serde(rename = "s")]
    #[schemars(title = "snapshot")]
    Snapshot(L2BookSnapshot),
    #[serde(rename = "d")]
    #[schemars(title = "diff")]
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L2BookSnapshotRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_l2_book_updates",
    response = "L2BookUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeL2BookUpdatesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExternalL2BookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryExternalL2BookSnapshot {
    pub market_id: MarketId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L3BookSnapshot {
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    pub sequence: SequenceIdAndNumber,
    #[serde(rename = "b")]
    #[schemars(title = "bids")]
    pub bids: Vec<(u64, Decimal, Decimal)>,
    #[serde(rename = "a")]
    #[schemars(title = "asks")]
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeCandlesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeManyCandlesRequest {
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<MarketId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    pub candle_width: CandleWidth,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_trades",
    response = "Trade",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeTradesRequest {
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Candle {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "w")]
    #[schemars(title = "width")]
    pub width: CandleWidth,
    #[serde(rename = "o")]
    #[schemars(title = "open")]
    pub open: Decimal,
    #[serde(rename = "h")]
    #[schemars(title = "high")]
    pub high: Decimal,
    #[serde(rename = "l")]
    #[schemars(title = "low")]
    pub low: Decimal,
    #[serde(rename = "c")]
    #[schemars(title = "close")]
    pub close: Decimal,
    #[serde(rename = "v")]
    #[schemars(title = "volume")]
    pub volume: Decimal,
    #[serde(rename = "bv")]
    #[schemars(title = "buy_volume")]
    pub buy_volume: Decimal,
    #[serde(rename = "av")]
    #[schemars(title = "sell_volume")]
    pub sell_volume: Decimal,
    #[serde(rename = "mo", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_open")]
    pub mid_open: Option<Decimal>,
    #[serde(rename = "mc", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_close")]
    pub mid_close: Option<Decimal>,
    #[serde(rename = "mh", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_high")]
    pub mid_high: Option<Decimal>,
    #[serde(rename = "ml", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_low")]
    pub mid_low: Option<Decimal>,
    #[serde(rename = "bo", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_open")]
    pub bid_open: Option<Decimal>,
    #[serde(rename = "bc", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_close")]
    pub bid_close: Option<Decimal>,
    #[serde(rename = "bh", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_high")]
    pub bid_high: Option<Decimal>,
    #[serde(rename = "bl", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_low")]
    pub bid_low: Option<Decimal>,
    #[serde(rename = "ao", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_open")]
    pub ask_open: Option<Decimal>,
    #[serde(rename = "ac", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_close")]
    pub ask_close: Option<Decimal>,
    #[serde(rename = "ah", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_high")]
    pub ask_high: Option<Decimal>,
    #[serde(rename = "al", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_low")]
    pub ask_low: Option<Decimal>,
}

impl Candle {
    pub fn from_candle_v1(
        market_id: MarketId,
        candle: CandleV1,
        width: CandleWidth,
    ) -> Self {
        Self {
            market_id: Some(market_id),
            symbol: None,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Trade {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "d")]
    #[schemars(title = "direction")]
    pub direction: Option<Dir>, // maker dir
    #[serde(rename = "p")]
    #[schemars(title = "price")]
    pub price: Decimal,
    #[serde(rename = "q")]
    #[schemars(title = "size")]
    pub size: Decimal,
}

impl Trade {
    pub fn new(
        market_id: Option<MarketId>,
        symbol: Option<String>,
        price: Decimal,
        size: Decimal,
        direction: Option<Dir>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            market_id,
            symbol,
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            direction,
            price,
            size,
        }
    }
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "market_status", response = "MarketStatus")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketStatus {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    pub is_trading: Option<bool>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "ticker", response = "Ticker")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TickerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ticker {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "o")]
    #[schemars(title = "open_24h")]
    pub open_24h: Option<Decimal>,
    #[serde(rename = "v")]
    #[schemars(title = "volume_24h")]
    pub volume_24h: Option<Decimal>,
    #[serde(rename = "l")]
    #[schemars(title = "low_24h")]
    pub low_24h: Option<Decimal>,
    #[serde(rename = "h")]
    #[schemars(title = "high_24h")]
    pub high_24h: Option<Decimal>,
    #[serde(rename = "vm")]
    #[schemars(title = "volume_30d")]
    pub volume_30d: Option<Decimal>,
    #[serde(rename = "oi")]
    #[schemars(title = "open_interest")]
    pub open_interest: Option<Decimal>,
}

/// Ticker updates are not strongly ordered because the data is considered
/// more casual.  You may receive diffs or snapshots slightly out of order.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "ticker",
    response = "TickerUpdate",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeTickersRequest {
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<MarketId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
pub enum TickerUpdate {
    #[serde(rename = "s")]
    #[schemars(rename = "snapshot")]
    Snapshot(Ticker),
    #[serde(rename = "d")]
    #[schemars(rename = "diff")]
    Diff(TickerDiff),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TickerDiff {
    #[serde(rename = "m", default)]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default)]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "o", default)]
    #[schemars(title = "open_24h")]
    pub open_24h: Option<Decimal>,
    #[serde(rename = "v", default)]
    #[schemars(title = "volume_24h")]
    pub volume_24h: Option<Decimal>,
    #[serde(rename = "l", default)]
    #[schemars(title = "low_24h")]
    pub low_24h: Option<Decimal>,
    #[serde(rename = "h", default)]
    #[schemars(title = "high_24h")]
    pub high_24h: Option<Decimal>,
    #[serde(rename = "vm", default)]
    #[schemars(title = "volume_30d")]
    pub volume_30d: Option<Decimal>,
    #[serde(rename = "oi", default)]
    #[schemars(title = "open_interest")]
    pub open_interest: Option<Decimal>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "subscribe_liquidations", response = "Liquidation")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct SubscribeLiquidationsRequest {
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<MarketId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Liquidation {
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "market_id")]
    pub market_id: Option<MarketId>,
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "d")]
    #[schemars(title = "direction")]
    pub direction: Dir,
    #[serde(rename = "p")]
    #[schemars(title = "price")]
    pub price: Decimal,
    #[serde(rename = "q")]
    #[schemars(title = "size")]
    pub size: Decimal,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "exchange_specific_fields",
    response = "ExchangeSpecificFields"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct ExchangeSpecificFieldsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<MarketId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// If None, subscribe from all exchange-specific fields on the feed
    pub fields: Option<Vec<String>>,
}

// CR alee: we are a somewhat locked to JSON here
#[derive(
    Default,
    Debug,
    Deref,
    DerefMut,
    Clone,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    JsonSchema,
)]
#[serde(transparent)]
pub struct ExchangeSpecificFields(pub BTreeMap<String, serde_json::Value>);
