use crate::{
    symbology::{ExecutionVenue, TradableProduct},
    AccountId, Dir, OrderId, UserId,
};
use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, BoolFromInt};
use strum::FromRepr;
use uuid::Uuid;

#[derive(
    Debug,
    Display,
    FromStr,
    FromRepr,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum FillKind {
    Normal = 0,
    Reversal = 1,
    Correction = 2,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Fill {
    #[serde(rename = "id")]
    #[schemars(title = "fill_id")]
    pub fill_id: Uuid,
    #[serde(rename = "k")]
    #[schemars(title = "fill_kind")]
    pub fill_kind: FillKind,
    #[serde(rename = "x")]
    #[schemars(title = "execution_venue")]
    pub execution_venue: ExecutionVenue,
    #[serde(rename = "xid")]
    #[schemars(title = "exchange_fill_id")]
    pub exchange_fill_id: Option<String>,
    #[serde(rename = "oid")]
    #[schemars(title = "order_id")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "u")]
    #[schemars(title = "trader")]
    pub trader: Option<UserId>,
    #[serde(rename = "a")]
    #[schemars(title = "account")]
    pub account: Option<AccountId>,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: TradableProduct,
    #[serde(rename = "d")]
    #[schemars(title = "direction")]
    pub dir: Dir,
    #[serde(rename = "q")]
    #[schemars(title = "quantity")]
    pub quantity: Decimal,
    #[serde(rename = "p")]
    #[schemars(title = "price")]
    pub price: Decimal,
    #[serde(rename = "t")]
    #[serde_as(as = "Option<BoolFromInt>")]
    #[schemars(title = "is_taker", with = "isize")]
    pub is_taker: Option<bool>,
    #[serde(rename = "f")]
    #[schemars(title = "fee")]
    pub fee: Option<Decimal>,
    /// Fee currency, if different from the price currency
    #[serde(rename = "fu")]
    #[schemars(title = "fee_currency")]
    pub fee_currency: Option<String>,
    /// When Architect received the fill, if realtime
    #[serde(rename = "ats")]
    #[schemars(title = "recv_time")]
    pub recv_time: Option<i64>,
    #[serde(rename = "atn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: Option<u32>,
    /// When the cpty claims the trade happened
    #[serde(rename = "ts")]
    #[schemars(title = "trade_time")]
    pub trade_time: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "trade_time_ns")]
    pub trade_time_ns: u32,
}

impl Fill {
    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        if let Some(recv_time) = self.recv_time {
            DateTime::from_timestamp(recv_time, self.recv_time_ns.unwrap_or(0))
        } else {
            None
        }
    }

    pub fn trade_time(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.trade_time, self.trade_time_ns)
    }
}

/// Fills which we received but couldn't parse fully,
/// return details best effort
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AberrantFill {
    #[serde(rename = "id")]
    #[schemars(title = "fill_id")]
    pub fill_id: Uuid,
    #[serde(rename = "k")]
    #[schemars(title = "fill_kind")]
    pub fill_kind: Option<FillKind>,
    #[serde(rename = "x")]
    #[schemars(title = "execution_venue")]
    pub execution_venue: ExecutionVenue,
    #[serde(rename = "xid")]
    #[schemars(title = "exchange_fill_id")]
    pub exchange_fill_id: Option<String>,
    #[serde(rename = "oid")]
    #[schemars(title = "order_id")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "u")]
    #[schemars(title = "trader")]
    pub trader: Option<UserId>,
    #[serde(rename = "a")]
    #[schemars(title = "account")]
    pub account: Option<AccountId>,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "d")]
    #[schemars(title = "direction")]
    pub dir: Option<Dir>,
    #[serde(rename = "q")]
    #[schemars(title = "quantity")]
    pub quantity: Option<Decimal>,
    #[serde(rename = "p")]
    #[schemars(title = "price")]
    pub price: Option<Decimal>,
    #[serde(rename = "f")]
    #[schemars(title = "fee")]
    pub fee: Option<Decimal>,
    #[serde(rename = "fu")]
    #[schemars(title = "fee_currency")]
    pub fee_currency: Option<String>,
    #[serde(rename = "ats")]
    #[schemars(title = "recv_time")]
    pub recv_time: Option<i64>,
    #[serde(rename = "atn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: Option<u32>,
    #[serde(rename = "ts")]
    #[schemars(title = "trade_time")]
    pub trade_time: Option<i64>,
    #[serde(rename = "tn")]
    #[schemars(title = "trade_time_ns")]
    pub trade_time_ns: Option<u32>,
}

impl AberrantFill {
    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        if let Some(recv_time) = self.recv_time {
            DateTime::from_timestamp(recv_time, self.recv_time_ns.unwrap_or(0))
        } else {
            None
        }
    }

    pub fn trade_time(&self) -> Option<DateTime<Utc>> {
        if let Some(trade_time) = self.trade_time {
            DateTime::from_timestamp(trade_time, self.trade_time_ns.unwrap_or(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use rust_decimal_macros::dec;
    use uuid::uuid;

    #[test]
    fn test_fill_json() {
        let recv_time: DateTime<Utc> = "2021-01-01T00:00:00Z".parse().unwrap();
        let trade_time: DateTime<Utc> = "2021-01-01T00:00:01Z".parse().unwrap();
        let fill = Fill {
            fill_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
            fill_kind: FillKind::Normal,
            execution_venue: "BINANCE".into(),
            exchange_fill_id: Some("123456".to_string()),
            order_id: Some(OrderId::nil(100)),
            trader: Some(UserId::anonymous()),
            account: Some(AccountId::nil()),
            symbol: "BTC-USD".to_string().into(),
            dir: Dir::Buy,
            quantity: dec!(1.5),
            price: dec!(50000),
            is_taker: Some(true),
            fee: Some(dec!(0.001)),
            fee_currency: Some("BTC".to_string()),
            recv_time: Some(recv_time.timestamp()),
            recv_time_ns: Some(recv_time.timestamp_subsec_nanos()),
            trade_time: trade_time.timestamp(),
            trade_time_ns: trade_time.timestamp_subsec_nanos(),
        };
        insta::assert_json_snapshot!(fill, @r###"
        {
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "k": 0,
          "x": "BINANCE",
          "xid": "123456",
          "oid": "100",
          "u": "00000000-0000-0000-0000-000000000000",
          "a": "00000000-0000-0000-0000-000000000000",
          "s": "BTC-USD",
          "d": "BUY",
          "q": "1.5",
          "p": "50000",
          "t": 1,
          "f": "0.001",
          "fu": "BTC",
          "ats": 1609459200,
          "atn": 0,
          "ts": 1609459201,
          "tn": 0
        }
        "###);
    }
}
