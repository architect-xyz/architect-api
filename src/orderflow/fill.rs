use crate::{symbology::MarketId, Dir, OrderId};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use uuid::{uuid, Uuid};

static FILL_NS: Uuid = uuid!("c5e7ca09-1223-4f3a-9ba9-609b8d07629d");

/// The ID of a fill
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pack,
    FromValue,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar), graphql(transparent))]
pub struct FillId(Uuid);

impl FillId {
    /// This function will always generate the same UUID for the identifier provided
    pub fn from_id(id: &str) -> Self {
        FillId(Uuid::new_v5(&FILL_NS, id.as_bytes()))
    }

    pub fn nil() -> Self {
        FillId(Uuid::nil())
    }
}

impl Default for FillId {
    fn default() -> Self {
        FillId(Uuid::new_v4())
    }
}

impl Display for FillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for FillId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Fill {
    pub kind: FillKind,
    pub fill_id: FillId,
    /// Corresponding order ID, if the order originated from Architect
    pub order_id: Option<OrderId>,
    pub market: MarketId,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    /// When Architect received the fill, if realtime
    pub recv_time: Option<DateTime<Utc>>,
    /// When the cpty claims the trade happened
    pub trade_time: DateTime<Utc>,
}

impl Fill {
    pub fn into_aberrant(self) -> AberrantFill {
        AberrantFill {
            kind: Some(self.kind),
            fill_id: self.fill_id,
            order_id: self.order_id,
            market: Some(self.market),
            quantity: Some(self.quantity),
            price: Some(self.price),
            dir: Some(self.dir),
            recv_time: self.recv_time,
            trade_time: Some(self.trade_time),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Pack, Serialize, Deserialize, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(u8)]
pub enum FillKind {
    Normal,
    Reversal,
    Correction,
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for FillKind {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, ValueRef};
        let value_ref = match self {
            FillKind::Normal => ValueRef::Text("Normal".as_ref()),
            FillKind::Reversal => ValueRef::Text("Reversal".as_ref()),
            FillKind::Correction => ValueRef::Text("Correction".as_ref()),
        };
        Ok(ToSqlOutput::Borrowed(value_ref))
    }
}

/// Fills which we received but couldn't parse fully, return details
/// best effort
#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AberrantFill {
    pub kind: Option<FillKind>,
    pub fill_id: FillId,
    pub order_id: Option<OrderId>,
    pub market: Option<MarketId>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub dir: Option<Dir>,
    pub recv_time: Option<DateTime<Utc>>,
    pub trade_time: Option<DateTime<Utc>>,
}
