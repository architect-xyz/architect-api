use super::order_types::*;
use crate::{symbology::ExecutionVenue, AccountId, Dir, OrderId, UserId};
use chrono::{DateTime, Utc};
use derive_more::Display;
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Order {
    pub id: OrderId,
    #[serde(rename = "pid")]
    #[schemars(title = "parent_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<OrderId>,
    #[serde(rename = "ts")]
    #[schemars(title = "recv_time")]
    pub recv_time: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: u32,
    #[serde(rename = "o")]
    #[schemars(title = "status")]
    pub status: OrderStatus,
    #[serde(rename = "r")]
    #[schemars(title = "reject_reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<OrderRejectReason>,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: String,
    #[serde(rename = "u")]
    #[schemars(title = "trader")]
    pub trader: UserId,
    #[serde(rename = "a")]
    #[schemars(title = "account")]
    pub account: AccountId,
    #[serde(rename = "d")]
    #[schemars(title = "dir")]
    pub dir: Dir,
    #[serde(rename = "q")]
    #[schemars(title = "quantity")]
    pub quantity: Decimal,
    #[serde(rename = "xq")]
    #[schemars(title = "filled_quantity")]
    pub filled_quantity: Decimal,
    #[serde(rename = "xp")]
    #[schemars(title = "average_fill_price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<Decimal>,
    #[serde(flatten)]
    pub order_type: OrderType,
    #[serde(rename = "tif")]
    #[schemars(title = "time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "src")]
    #[schemars(title = "source")]
    pub source: OrderSource,
    #[serde(rename = "x")]
    #[schemars(title = "execution_venue")]
    pub execution_venue: ExecutionVenue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    GoodTilCancel,
    #[serde(rename = "gtd")]
    GoodTilDate(DateTime<Utc>),
    /// Day order--the specific time which this expires
    /// will be dependent on the venue
    #[serde(rename = "day")]
    GoodTilDay,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    FillOrKill,
}

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum OrderSource {
    API = 0,
    GUI = 1,
    Algo = 2,
    External = 3,
    CLI = 4,
    Telegram = 5,
    #[serde(other)]
    Other = 255,
}

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, JsonSchema_repr,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum OrderStatus {
    Pending = 0,
    Acked = 1,
    Rejected = 2,
    Open = 3,
    Out = 4,
    Canceling = 128,
    Canceled = 129,
    Stale = 254,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct OrderAck {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderReject {
    pub order_id: OrderId,
    pub reason: OrderRejectReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderRejectReason {
    DuplicateOrderId,
    NotAuthorized,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct OrderOut {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct OrderStale {
    pub order_id: OrderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oms::PlaceOrderRequest;
    use rust_decimal_macros::dec;

    #[test]
    fn test_place_order_request_json() {
        #[rustfmt::skip]
        let por: PlaceOrderRequest = serde_json::from_str(r#"
            {
                "id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:123",
                "s": "BTC Crypto/USD",
                "d": "buy",
                "q": "100",
                "u": "trader1",
                "a": "account1",
                "k": "limit",
                "p": "4500",
                "po": true,
                "tif": {
                    "gtd": "2025-01-05T04:20:00Z"
                } 
            }
        "#).unwrap();
        assert_eq!(
            por,
            PlaceOrderRequest {
                id: Some(OrderId {
                    seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                    seqno: 123
                }),
                symbol: "BTC Crypto/USD".into(),
                dir: Dir::Buy,
                quantity: dec!(100),
                trader: Some("trader1".into()),
                account: Some("account1".into()),
                order_type: OrderType::Limit(LimitOrderType {
                    limit_price: dec!(4500),
                    post_only: true,
                }),
                time_in_force: TimeInForce::GoodTilDate(
                    "2025-01-05T04:20:00Z".parse().unwrap()
                ),
                execution_venue: None,
            }
        );
    }

    #[test]
    fn test_order_json() {
        let recv_time: DateTime<Utc> = "2025-01-01T04:20:00Z".parse().unwrap();
        insta::assert_json_snapshot!(Order {
            id: OrderId {
                seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                seqno: 123
            },
            parent_id: None,
            recv_time: recv_time.timestamp(),
            recv_time_ns: recv_time.timestamp_subsec_nanos(),
            status: OrderStatus::Out,
            reject_reason: Some(OrderRejectReason::DuplicateOrderId),
            symbol: "BTC Crypto/USD".into(),
            trader: UserId::anonymous(),
            account: AccountId::nil(),
            dir: Dir::Buy,
            quantity: dec!(100),
            filled_quantity: dec!(0),
            average_fill_price: None,
            order_type: OrderType::Limit(LimitOrderType {
                limit_price: dec!(4500),
                post_only: false,
            }),
            time_in_force: TimeInForce::GoodTilCancel,
            source: OrderSource::API,
            execution_venue: "BINANCE".into(), 
        }, @r###"
        {
          "id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:123",
          "ts": 1735705200,
          "tn": 0,
          "o": 4,
          "r": "duplicate_order_id",
          "s": "BTC Crypto/USD",
          "u": "00000000-0000-0000-0000-000000000000",
          "a": "00000000-0000-0000-0000-000000000000",
          "d": "buy",
          "q": "100",
          "xq": "0",
          "k": "limit",
          "p": "4500",
          "po": false,
          "tif": "gtc",
          "src": 0,
          "x": "BINANCE"
        }
        "###);
        let recv_time: DateTime<Utc> = "2025-01-01T04:20:00Z".parse().unwrap();
        insta::assert_json_snapshot!(Order {
            id: OrderId::nil(123),
            parent_id: Some(OrderId {
                seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                seqno: 456
            }),
            recv_time: recv_time.timestamp(),
            recv_time_ns: recv_time.timestamp_subsec_nanos(),
            status: OrderStatus::Open,
            reject_reason: None,
            symbol: "ETH Crypto/USD".into(),
            trader: UserId::anonymous(),
            account: AccountId::nil(),
            dir: Dir::Sell,
            quantity: dec!(0.7050),
            filled_quantity: dec!(0.7050),
            average_fill_price: Some(dec!(4250)),
            order_type: OrderType::StopLossLimit(StopLossLimitOrderType {
                limit_price: dec!(4500),
                trigger_price: dec!(4000),
            }),
            time_in_force: TimeInForce::GoodTilDate(
                "2025-01-05T04:20:00Z".parse().unwrap()
            ),
            source: OrderSource::Telegram,
            execution_venue: "BINANCE".into(),
        }, @r###"
        {
          "id": "123",
          "pid": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:456",
          "ts": 1735705200,
          "tn": 0,
          "o": 3,
          "s": "ETH Crypto/USD",
          "u": "00000000-0000-0000-0000-000000000000",
          "a": "00000000-0000-0000-0000-000000000000",
          "d": "sell",
          "q": "0.7050",
          "xq": "0.7050",
          "xp": "4250",
          "k": "stop_loss_limit",
          "p": "4500",
          "tp": "4000",
          "tif": {
            "gtd": "2025-01-05T04:20:00Z"
          },
          "src": 5,
          "x": "BINANCE"
        }
        "###);
    }
}
