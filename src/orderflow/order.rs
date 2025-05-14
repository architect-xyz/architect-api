use super::order_types::*;
use crate::{
    symbology::{ExecutionVenue, TradableProduct},
    AccountId, Dir, OrderId, UserId,
};
use chrono::{DateTime, Utc};
use derive_more::{Display, FromStr};
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum::{FromRepr, IntoStaticStr};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
/// <!-- py: unflatten=k/order_type/OrderType, tag=k -->
pub struct Order {
    pub id: OrderId,
    #[serde(rename = "pid")]
    #[schemars(title = "parent_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<OrderId>,
    #[serde(rename = "eid")]
    #[schemars(title = "exchange_order_id")]
    pub exchange_order_id: Option<String>,
    #[serde(rename = "ts")]
    #[schemars(title = "recv_time")]
    pub recv_time: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: u32,
    #[serde(rename = "o")]
    #[schemars(title = "status")]
    pub status: OrderStatus,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "reject_reason")]
    pub reject_reason: Option<OrderRejectReason>,
    #[serde(rename = "rm", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "reject_message")]
    pub reject_message: Option<String>,
    #[serde(rename = "s")]
    #[schemars(title = "symbol")]
    pub symbol: TradableProduct,
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
    #[serde(rename = "ve")]
    #[schemars(title = "execution_venue")]
    pub execution_venue: ExecutionVenue,
    #[serde(rename = "ss", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "is_short_sale")]
    pub is_short_sale: Option<bool>,
}

impl Order {
    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.recv_time, self.recv_time_ns)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub id: OrderId,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "tn")]
    pub timestamp_ns: u32,
    #[serde(rename = "o")]
    pub status: Option<OrderStatus>,
    #[serde(rename = "r")]
    pub reject_reason: Option<OrderRejectReason>,
    #[serde(rename = "rm")]
    pub reject_message: Option<String>,
    #[serde(rename = "xq")]
    pub filled_quantity: Option<Decimal>,
    #[serde(rename = "xp")]
    pub average_fill_price: Option<Decimal>,
}

impl OrderUpdate {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}

#[derive(
    Debug, Clone, Copy, IntoStaticStr, Serialize, Deserialize, PartialEq, Eq, JsonSchema,
)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    #[strum(serialize = "GTC")]
    #[schemars(title = "GoodTilCancel")]
    GoodTilCancel,
    #[serde(rename = "GTD")]
    #[strum(serialize = "GTD")]
    #[schemars(title = "GoodTilDate")]
    GoodTilDate(DateTime<Utc>),
    /// Day order--the specific time which this expires
    /// will be dependent on the venue
    #[serde(rename = "DAY")]
    #[strum(serialize = "DAY")]
    #[schemars(title = "GoodTilDay")]
    GoodTilDay,
    #[serde(rename = "IOC")]
    #[strum(serialize = "IOC")]
    #[schemars(title = "ImmediateOrCancel")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    #[strum(serialize = "FOK")]
    #[schemars(title = "FillOrKill")]
    FillOrKill,
    #[serde(rename = "ATO")]
    #[strum(serialize = "ATO")]
    #[schemars(title = "AtTheOpen")]
    AtTheOpen,
    #[serde(rename = "ATC")]
    #[strum(serialize = "ATC")]
    #[schemars(title = "AtTheClose")]
    AtTheClose,
}

impl TimeInForce {
    pub fn good_til_date(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::GoodTilDate(dt) => Some(*dt),
            _ => None,
        }
    }
}

#[derive(
    Debug,
    Display,
    FromStr,
    Clone,
    Copy,
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum OrderSource {
    API = 0,
    GUI = 1,
    Algo = 2,
    Reconciled = 3,
    CLI = 4,
    Telegram = 5,
    #[serde(other)]
    Other = 255,
}

#[cfg(feature = "postgres")]
crate::to_sql_display!(OrderSource);

#[derive(
    Debug,
    Display,
    FromStr,
    FromRepr,
    Clone,
    Copy,
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum OrderStatus {
    Pending = 0,
    Open = 1,
    Rejected = 2,
    Out = 127,
    Canceling = 128,
    Canceled = 129,
    ReconciledOut = 130,
    Stale = 254,
    Unknown = 255,
}

impl OrderStatus {
    pub fn is_alive(&self) -> bool {
        match self {
            Self::Pending
            | Self::Open
            | Self::Canceling
            | Self::Stale
            | Self::Unknown => true,
            Self::Out | Self::Canceled | Self::Rejected | Self::ReconciledOut => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct OrderAck {
    #[serde(rename = "id")]
    #[schemars(title = "order_id")]
    pub order_id: OrderId,
    #[serde(rename = "eid")]
    #[schemars(title = "exchange_order_id")]
    pub exchange_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderReject {
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "r")]
    #[schemars(title = "reject_reason")]
    pub reason: OrderRejectReason,
    #[serde(rename = "rm", skip_serializing_if = "Option::is_none")]
    #[schemars(title = "message")]
    pub message: Option<String>,
}

impl OrderReject {
    pub fn to_error_string(&self) -> String {
        format!(
            "order {} rejected: {} ({})",
            self.order_id,
            self.message.as_deref().unwrap_or("--"),
            self.reason
        )
    }
}

#[derive(Debug, Display, FromStr, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "TEXT"))]
pub enum OrderRejectReason {
    DuplicateOrderId,
    NotAuthorized,
    NoExecutionVenue,
    NoAccount,
    NoCpty,
    UnsupportedOrderType,
    UnsupportedExecutionVenue,
    InsufficientCash,
    InsufficientMargin,
    NotEasyToBorrow,
    #[serde(other)]
    Unknown,
}

#[cfg(feature = "postgres")]
crate::to_sql_display!(OrderRejectReason);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct OrderCanceling {
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "xid", skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct OrderCanceled {
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "xid", skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct OrderOut {
    #[serde(rename = "id")]
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct OrderStale {
    #[serde(rename = "id")]
    pub order_id: OrderId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{oms::PlaceOrderRequest, AccountIdOrName, AccountName, TraderIdOrEmail};
    use rust_decimal_macros::dec;

    #[test]
    fn test_place_order_request_json() {
        #[rustfmt::skip]
        let por: PlaceOrderRequest = serde_json::from_str(r#"
            {
                "id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:123",
                "s": "BTC Crypto/USD",
                "d": "BUY",
                "q": "100",
                "u": "trader1",
                "a": "COINBASE:TEST",
                "k": "LIMIT",
                "p": "4500",
                "po": true,
                "tif": {
                    "GTD": "2025-01-05T04:20:00Z"
                } 
            }
        "#).unwrap();
        let trader: UserId = "trader1".parse().unwrap();
        assert_eq!(
            por,
            PlaceOrderRequest {
                id: Some(OrderId {
                    seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                    seqno: 123
                }),
                parent_id: None,
                symbol: "BTC Crypto/USD".into(),
                dir: Dir::Buy,
                quantity: dec!(100),
                trader: Some(TraderIdOrEmail::Id(trader)),
                account: Some(AccountIdOrName::Name(
                    AccountName::new("COINBASE", "TEST").unwrap()
                )),
                order_type: OrderType::Limit(LimitOrderType {
                    limit_price: dec!(4500),
                    post_only: true,
                }),
                time_in_force: TimeInForce::GoodTilDate(
                    "2025-01-05T04:20:00Z".parse().unwrap()
                ),
                source: None,
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
            exchange_order_id: None,
            recv_time: recv_time.timestamp(),
            recv_time_ns: recv_time.timestamp_subsec_nanos(),
            status: OrderStatus::Out,
            reject_reason: Some(OrderRejectReason::DuplicateOrderId),
            reject_message: None,
            symbol: "BTC Crypto/USD".parse().unwrap(),
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
            is_short_sale: None,
        }, @r###"
        {
          "id": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:123",
          "eid": null,
          "ts": 1735705200,
          "tn": 0,
          "o": 127,
          "r": "DuplicateOrderId",
          "s": "BTC Crypto/USD",
          "u": "00000000-0000-0000-0000-000000000000",
          "a": "00000000-0000-0000-0000-000000000000",
          "d": "BUY",
          "q": "100",
          "xq": "0",
          "k": "LIMIT",
          "p": "4500",
          "po": false,
          "tif": "GTC",
          "src": 0,
          "ve": "BINANCE"
        }
        "###);
        let recv_time: DateTime<Utc> = "2025-01-01T04:20:00Z".parse().unwrap();
        insta::assert_json_snapshot!(Order {
            id: OrderId::nil(123),
            parent_id: Some(OrderId {
                seqid: "d3f97244-78e6-4549-abf6-90adfe0ab7fe".parse().unwrap(),
                seqno: 456
            }),
            exchange_order_id: None,
            recv_time: recv_time.timestamp(),
            recv_time_ns: recv_time.timestamp_subsec_nanos(),
            status: OrderStatus::Open,
            reject_reason: None,
            reject_message: None,
            symbol: "ETH Crypto/USD".parse().unwrap(),
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
            is_short_sale: None,
        }, @r###"
        {
          "id": "123",
          "pid": "d3f97244-78e6-4549-abf6-90adfe0ab7fe:456",
          "eid": null,
          "ts": 1735705200,
          "tn": 0,
          "o": 1,
          "s": "ETH Crypto/USD",
          "u": "00000000-0000-0000-0000-000000000000",
          "a": "00000000-0000-0000-0000-000000000000",
          "d": "SELL",
          "q": "0.7050",
          "xq": "0.7050",
          "xp": "4250",
          "k": "STOP_LOSS_LIMIT",
          "p": "4500",
          "tp": "4000",
          "tif": {
            "GTD": "2025-01-05T04:20:00Z"
          },
          "src": 5,
          "ve": "BINANCE"
        }
        "###);
    }
}
