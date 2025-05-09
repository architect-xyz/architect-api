use crate::{
    symbology::MarketdataVenue,
    utils::{chrono::DateTimeOrUtc, pagination::OffsetAndLimit},
    Dir, DirPair, SequenceIdAndNumber,
};
use chrono::{DateTime, NaiveDate, Utc};
use derive::grpc;
use derive_more::Deref;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use strum::EnumString;

pub mod candle_width;
pub use candle_width::CandleWidth;
pub mod options_marketdata;

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshot", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshotRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "l1_book_snapshots", response = "L1BookSnapshot")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshotsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct L1BookSnapshot {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    /// Time that Architect feed received the message;
    /// only set if streaming from direct L1 feeds
    #[serde(rename = "rt", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "recv_time")]
    pub recv_time: Option<i64>,
    #[serde(rename = "rtn", default, skip_serializing_if = "Option::is_none")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: Option<u32>,
    /// (price, quantity)
    #[serde(rename = "b")]
    #[schemars(title = "best_bid")]
    pub best_bid: Option<(Decimal, Decimal)>,
    /// (price, quantity)
    #[serde(rename = "a")]
    #[schemars(title = "best_ask")]
    pub best_ask: Option<(Decimal, Decimal)>,
}

impl L1BookSnapshot {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        chrono::DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }

    pub fn bbo(&self) -> DirPair<Option<Decimal>> {
        DirPair {
            buy: self.best_bid.map(|(px, _)| px),
            sell: self.best_ask.map(|(px, _)| px),
        }
    }

    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        match (self.recv_time, self.recv_time_ns) {
            (Some(ts), Some(ns)) => chrono::DateTime::from_timestamp(ts, ns),
            _ => None,
        }
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
/// # use architect_api::marketdata::*;
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
/// <!-- py: tag=t -->
pub enum L2BookUpdate {
    #[serde(rename = "s")]
    #[schemars(title = "Snapshot|L2BookSnapshot")]
    Snapshot(L2BookSnapshot),
    #[serde(rename = "d")]
    #[schemars(title = "Diff|L2BookDiff")]
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
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
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
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
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
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
    /// If None, subscribe from all candle widths on the feed
    pub candle_widths: Option<Vec<CandleWidth>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    pub candle_width: CandleWidth,
}

/// Subscribe to the current candle.  This allows you to display
/// the most recent/building candle live in a UI, for example.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "subscribe_current_candles",
    response = "Candle",
    server_streaming
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubscribeCurrentCandlesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
    pub candle_width: CandleWidth,
    /// If None, send the current candle on every trade or candle tick.
    /// Otherwise, send a candle every `tick_period_ms`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tick_period_ms: Option<u32>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct Candle {
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(rename = "w")]
    #[schemars(title = "width")]
    pub width: CandleWidth,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "o")]
    #[schemars(title = "open")]
    pub open: Option<Decimal>,
    #[serde(rename = "h")]
    #[schemars(title = "high")]
    pub high: Option<Decimal>,
    #[serde(rename = "l")]
    #[schemars(title = "low")]
    pub low: Option<Decimal>,
    #[serde(rename = "c")]
    #[schemars(title = "close")]
    pub close: Option<Decimal>,
    #[serde(rename = "v")]
    #[schemars(title = "volume")]
    pub volume: Decimal,
    #[serde(rename = "bv")]
    #[schemars(title = "buy_volume")]
    pub buy_volume: Decimal,
    #[serde(rename = "av")]
    #[schemars(title = "sell_volume")]
    pub sell_volume: Decimal,
    #[serde(default, rename = "mo", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_open")]
    pub mid_open: Option<Decimal>,
    #[serde(default, rename = "mc", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_close")]
    pub mid_close: Option<Decimal>,
    #[serde(default, rename = "mh", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_high")]
    pub mid_high: Option<Decimal>,
    #[serde(default, rename = "ml", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "mid_low")]
    pub mid_low: Option<Decimal>,
    #[serde(default, rename = "bo", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_open")]
    pub bid_open: Option<Decimal>,
    #[serde(default, rename = "bc", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_close")]
    pub bid_close: Option<Decimal>,
    #[serde(default, rename = "bh", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_high")]
    pub bid_high: Option<Decimal>,
    #[serde(default, rename = "bl", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "bid_low")]
    pub bid_low: Option<Decimal>,
    #[serde(default, rename = "ao", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_open")]
    pub ask_open: Option<Decimal>,
    #[serde(default, rename = "ac", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_close")]
    pub ask_close: Option<Decimal>,
    #[serde(default, rename = "ah", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_high")]
    pub ask_high: Option<Decimal>,
    #[serde(default, rename = "al", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "ask_low")]
    pub ask_low: Option<Decimal>,
}

impl Candle {
    pub fn default(timestamp: DateTime<Utc>, width: CandleWidth, symbol: String) -> Self {
        Self {
            timestamp: timestamp.timestamp(),
            timestamp_ns: timestamp.timestamp_subsec_nanos(),
            width,
            symbol,
            open: None,
            high: None,
            low: None,
            close: None,
            volume: dec!(0),
            buy_volume: dec!(0),
            sell_volume: dec!(0),
            mid_open: None,
            mid_close: None,
            mid_high: None,
            mid_low: None,
            bid_open: None,
            bid_close: None,
            bid_high: None,
            bid_low: None,
            ask_open: None,
            ask_close: None,
            ask_high: None,
            ask_low: None,
        }
    }

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

// Query historical candles for a single market.
#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "historical_candles",
    response = "HistoricalCandlesResponse"
)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalCandlesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
    pub candle_width: CandleWidth,
    #[serde_as(as = "DateTimeOrUtc")]
    #[schemars(with = "DateTimeOrUtc")]
    pub start_date: DateTime<Utc>,
    #[serde_as(as = "DateTimeOrUtc")]
    #[schemars(with = "DateTimeOrUtc")]
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoricalCandlesResponse {
    pub candles: Vec<Candle>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    /// If None, subscribe from all symbols on the feed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Trade {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
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
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "market_status", response = "MarketStatus")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct MarketStatus {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    pub is_trading: Option<bool>,
    pub is_quoting: Option<bool>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "ticker", response = "Ticker")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TickerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    pub symbol: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct TickerValues {
    #[serde(rename = "bp")]
    #[schemars(title = "bid_price")]
    pub bid_price: Option<Decimal>,
    #[serde(rename = "bs")]
    #[schemars(title = "bid_size")]
    pub bid_size: Option<Decimal>,
    #[serde(rename = "ap")]
    #[schemars(title = "ask_price")]
    pub ask_price: Option<Decimal>,
    #[serde(rename = "as")]
    #[schemars(title = "ask_size")]
    pub ask_size: Option<Decimal>,
    #[serde(rename = "p")]
    #[schemars(title = "last_price")]
    pub last_price: Option<Decimal>,
    #[serde(rename = "q")]
    #[schemars(title = "last_size")]
    pub last_size: Option<Decimal>,
    #[serde(rename = "xo")]
    #[schemars(title = "session_open")]
    pub session_open: Option<Decimal>,
    #[serde(rename = "xl")]
    #[schemars(title = "session_low")]
    pub session_low: Option<Decimal>,
    #[serde(rename = "xh")]
    #[schemars(title = "session_high")]
    pub session_high: Option<Decimal>,
    #[serde(rename = "xv")]
    #[schemars(title = "session_volume")]
    pub session_volume: Option<Decimal>,
    #[serde(rename = "o")]
    #[schemars(title = "open_24h")]
    pub open_24h: Option<Decimal>,
    #[serde(rename = "l")]
    #[schemars(title = "low_24h")]
    pub low_24h: Option<Decimal>,
    #[serde(rename = "h")]
    #[schemars(title = "high_24h")]
    pub high_24h: Option<Decimal>,
    #[serde(rename = "v")]
    #[schemars(title = "volume_24h")]
    pub volume_24h: Option<Decimal>,
    #[serde(rename = "vm")]
    #[schemars(title = "volume_30d")]
    pub volume_30d: Option<Decimal>,
    #[serde(rename = "oi")]
    #[schemars(title = "open_interest")]
    pub open_interest: Option<Decimal>,
    #[serde(rename = "sp")]
    #[schemars(title = "last_settlement_price")]
    pub last_settlement_price: Option<Decimal>,
    #[serde(rename = "sd")]
    #[schemars(title = "last_settlement_date")]
    pub last_settlement_date: Option<NaiveDate>,
    #[serde(rename = "mp")]
    #[schemars(title = "mark_price")]
    pub mark_price: Option<Decimal>,
    #[serde(rename = "ip")]
    #[schemars(title = "index_price")]
    pub index_price: Option<Decimal>,
    #[serde(rename = "fr")]
    #[schemars(title = "funding_rate")]
    pub funding_rate: Option<Decimal>,
    #[serde(rename = "ft")]
    #[schemars(title = "next_funding_time")]
    pub next_funding_time: Option<DateTime<Utc>>,
    pub market_cap: Option<Decimal>,
    pub price_to_earnings: Option<Decimal>,
    pub eps_adj: Option<Decimal>,
    pub shares_outstanding_weighted_adj: Option<Decimal>,
    pub dividend: Option<Decimal>,
    pub dividend_yield: Option<Decimal>,
    /*
    #[serde(rename = "dvex")]
    #[schemars(title = "dividend_ex_date")]
    pub dividend_ex_date: Option<String>,
    */
}

impl TickerValues {
    pub fn is_none(&self) -> bool {
        self.session_open.is_none()
            && self.session_low.is_none()
            && self.session_high.is_none()
            && self.open_24h.is_none()
            && self.low_24h.is_none()
            && self.high_24h.is_none()
            && self.volume_24h.is_none()
            && self.volume_30d.is_none()
            && self.open_interest.is_none()
            && self.last_settlement_price.is_none()
            && self.mark_price.is_none()
            && self.index_price.is_none()
            && self.funding_rate.is_none()
            && self.next_funding_time.is_none()
    }

    pub fn last_or_mid_price(&self) -> Option<Decimal> {
        self.last_price.or_else(|| {
            let bid_price = self.bid_price?;
            let ask_price = self.ask_price?;
            Some((bid_price + ask_price) / dec!(2))
        })
    }
}

#[derive(Debug, Deref, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ticker {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "ve")]
    #[schemars(title = "venue")]
    pub venue: MarketdataVenue,
    #[serde(rename = "ts")]
    #[schemars(title = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "timestamp_ns")]
    pub timestamp_ns: u32,
    #[serde(flatten)]
    #[deref]
    pub values: TickerValues,
}

impl Ticker {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

#[derive(Debug, Copy, Clone, EnumString, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum SortTickersBy {
    VolumeDesc,
    ChangeAsc,
    ChangeDesc,
    AbsChangeDesc,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "tickers", response = "TickersResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TickersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<MarketdataVenue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub pagination: OffsetAndLimit<SortTickersBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TickersResponse {
    pub tickers: Vec<Ticker>,
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
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t")]
/// <!-- py: tag=t -->
pub enum TickerUpdate {
    #[serde(rename = "s")]
    #[schemars(title = "Snapshot|Ticker")]
    Snapshot(Ticker),
    #[serde(rename = "d")]
    #[schemars(title = "Diff|Ticker")]
    Diff(Ticker),
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "subscribe_liquidations", response = "Liquidation")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct SubscribeLiquidationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Liquidation {
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
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
