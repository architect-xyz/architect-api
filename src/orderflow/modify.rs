use crate::{
    orderflow::{Order, OrderStatus, OrderType},
    OrderId,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Modify {
    #[serde(rename = "mid")]
    #[schemars(title = "modify_id")]
    pub modify_id: Uuid,
    #[serde(rename = "id")]
    #[schemars(title = "order_id")]
    pub order_id: OrderId,
    #[serde(rename = "nid")]
    #[schemars(title = "new_order_id")]
    /// The new order ID that will be assigned to the order after modification.
    pub new_order_id: OrderId,
    #[serde(rename = "p")]
    #[schemars(title = "new_price")]
    pub new_price: Option<Decimal>,
    #[serde(rename = "q")]
    #[schemars(title = "new_quantity")]
    pub new_quantity: Option<Decimal>,
    #[serde(rename = "ts")]
    #[schemars(title = "recv_time")]
    pub recv_time: i64,
    #[serde(rename = "tn")]
    #[schemars(title = "recv_time_ns")]
    pub recv_time_ns: u32,
    #[serde(rename = "o")]
    #[schemars(title = "status")]
    pub status: ModifyStatus,
    #[serde(rename = "r")]
    #[schemars(title = "reject_reason")]
    pub reject_reason: Option<String>,
}

impl Modify {
    pub fn recv_time(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.recv_time, self.recv_time_ns)
    }

    pub fn reject(&self, message: Option<String>) -> ModifyReject {
        ModifyReject { modify_id: self.modify_id, order_id: self.order_id, message }
    }

    /// Return a new order with the modified fields.
    pub fn modify(&self, mut order: Order) -> Result<Order> {
        order.id = self.new_order_id;
        order.quantity = self.new_quantity.unwrap_or(order.quantity);
        order.status = OrderStatus::Pending;

        if let Some(price) = self.new_price {
            match order.order_type {
                OrderType::Limit(ref mut l) => {
                    l.limit_price = price;
                }
                OrderType::StopLossLimit(ref mut p) => {
                    p.limit_price = price;
                }
                OrderType::TakeProfitLimit(ref mut p) => {
                    p.limit_price = price;
                }
                OrderType::Market => {
                    bail!("cannot modify market order with price")
                }
                OrderType::Bracket(ref mut b) => {
                    b.limit_price = price;
                }
            };
        }

        Ok(order)
    }
}

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, JsonSchema_repr,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum ModifyStatus {
    Pending = 0,
    Acked = 1,
    Rejected = 2,
    Out = 127,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct ModifyReject {
    #[serde(rename = "mid")]
    pub modify_id: Uuid,
    #[serde(rename = "id")]
    pub order_id: OrderId,
    #[serde(rename = "rm", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ModifyReject {
    pub fn to_error_string(&self) -> String {
        format!(
            "modify {} rejected: {}",
            self.modify_id,
            self.message.as_deref().unwrap_or("--")
        )
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyUpdate {
    pub modify_id: Uuid,
    pub order_id: OrderId,
    pub timestamp: i64,
    pub timestamp_ns: u32,
    pub status: Option<ModifyStatus>,
    pub reject_message: Option<String>,
}

impl ModifyUpdate {
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.timestamp, self.timestamp_ns)
    }
}
