use super::order_types::*;
use crate::{symbology::ExecutionVenue, AccountId, Dir, OrderId, UserId};
use chrono::{DateTime, Utc};
use derive_more::Display;
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Order {
    pub id: OrderId,
    pub parent_id: Option<OrderId>,
    pub recv_time: DateTime<Utc>,
    pub status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<OrderRejectReason>,
    pub symbol: String,
    pub trader: UserId,
    pub account: AccountId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub average_fill_price: Option<Decimal>,
    #[serde(flatten)]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub source: OrderSource,
    pub execution_venue: ExecutionVenue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    GoodTilCancel,
    GoodTilDate(DateTime<Utc>),
    /// Day order--the specific time which this expires
    /// will be dependent on the venue
    GoodTilDay,
    ImmediateOrCancel,
    FillOrKill,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema_repr)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum OrderSource {
    #[serde(rename = "api")]
    API,
    #[serde(rename = "gui")]
    GUI,
    Algo,
    External,
    #[serde(rename = "cli")]
    CLI,
    Telegram,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum OrderStatus {
    Pending,
    Acked,
    Rejected,
    Open,
    Out,
    Canceling,
    Canceled,
    Stale,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
pub struct OrderOut {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderStale {
    pub order_id: OrderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_place_order_request_json() {
        #[rustfmt::skip]
        let por: PlaceOrderRequest = serde_json::from_str(r#"
            {
                "id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:123",
                "symbol": "BTC Crypto/USD",
                "dir": "buy",
                "quantity": "100",
                "trader": "trader1",
                "account": "account1",
                "order_type": "limit",
                "limit_price": "4500",
                "post_only": true,
                "time_in_force": {
                    "good_til_date": "2025-01-05T04:20:00Z"
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
        insta::assert_json_snapshot!(Order {
            id: OrderId {
                seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                seqno: 123
            },
            parent_id: None,
            recv_time: "2025-01-01T04:20:00Z".parse().unwrap(),
            status: OrderStatus::Out,
            reject_reason: Some(OrderRejectReason::DuplicateOrderId),
            symbol: "BTC Crypto/USD".into(),
            trader: UserId::anonymous(),
            account: AccountId::nil(),
            dir: Dir::Buy,
            quantity: dec!(100), 
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
          "parent_id": null,
          "recv_time": "2025-01-01T04:20:00Z",
          "status": "out",
          "reject_reason": "duplicate_order_id",
          "symbol": "BTC Crypto/USD",
          "trader": "00000000-0000-0000-0000-000000000000",
          "account": "00000000-0000-0000-0000-000000000000",
          "dir": "buy",
          "quantity": "100",
          "order_type": "limit",
          "limit_price": "4500",
          "post_only": false,
          "time_in_force": "good_til_cancel",
          "source": "api",
          "execution_venue": "BINANCE"
        }
        "###);
        insta::assert_json_snapshot!(Order {
            id: OrderId::nil(123),
            parent_id: Some(OrderId {
                seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                seqno: 456
            }),
            recv_time: "2025-01-01T04:20:00Z".parse().unwrap(),
            status: OrderStatus::Open,
            reject_reason: None,
            symbol: "ETH Crypto/USD".into(),
            trader: UserId::anonymous(),
            account: AccountId::nil(),
            dir: Dir::Sell,
            quantity: dec!(0.7050),
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
          "parent_id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:456",
          "recv_time": "2025-01-01T04:20:00Z",
          "status": "open",
          "symbol": "ETH Crypto/USD",
          "trader": "00000000-0000-0000-0000-000000000000",
          "account": "00000000-0000-0000-0000-000000000000",
          "dir": "sell",
          "quantity": "0.7050",
          "order_type": "stop_loss_limit",
          "limit_price": "4500",
          "trigger_price": "4000",
          "time_in_force": {
            "good_til_date": "2025-01-05T04:20:00Z"
          },
          "source": "telegram",
          "execution_venue": "BINANCE"
        }
        "###);
    }
}
