use crate::symbology::VenueId;
#[cfg(feature = "netidx")]
use crate::{
    symbology::{MarketId, ProductId},
    AccountId, Dir, OrderId, UserId,
};
#[cfg(feature = "netidx")]
use anyhow::anyhow;
#[cfg(feature = "tokio-postgres")]
use bytes::BytesMut;
#[cfg(feature = "netidx")]
use chrono::{DateTime, Utc};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
#[cfg(feature = "netidx")]
use rust_decimal::Decimal;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[cfg(feature = "tokio-postgres")]
use std::error::Error;
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

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
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar), graphql(transparent))]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct FillId(Uuid);

impl FillId {
    /// This function will always generate the same UUID for the
    /// identifier bytes provided; venue is split out as an input
    /// to reduce the chance of accidental collision.
    pub fn from_id(venue: VenueId, id: &[u8]) -> Self {
        FillId(Uuid::new_v5(&venue, id))
    }

    pub fn nil() -> Self {
        FillId(Uuid::nil())
    }
}

#[cfg(feature = "sqlx")]
impl<'a> sqlx::Decode<'a, sqlx::Postgres> for FillId {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'a>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(FillId(Uuid::decode(value)?))
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

impl FromStr for FillId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(FillId)
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for FillId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

#[cfg(feature = "tokio-postgres")]
impl tokio_postgres::types::ToSql for FillId {
    tokio_postgres::types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        Uuid::accepts(ty)
    }
}

#[cfg(feature = "tokio-postgres")]
impl<'a> tokio_postgres::types::FromSql<'a> for FillId {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Uuid::from_sql(ty, raw).map(FillId)
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        Uuid::accepts(ty)
    }
}

#[cfg(feature = "netidx")]
#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Fee {
    pub amount: Decimal,
    pub fee_currency: ProductId,
}

#[cfg(feature = "netidx")]
#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct Fill {
    pub kind: FillKind,
    pub fill_id: FillId,
    /// Corresponding order ID, if the order originated from Architect
    pub order_id: Option<OrderId>,
    pub account_id: Option<AccountId>,
    pub market: MarketId,
    pub quantity: Decimal,
    pub price: Decimal,
    pub dir: Dir,
    pub is_maker: Option<bool>,
    /// When Architect received the fill, if realtime
    pub recv_time: Option<DateTime<Utc>>,
    /// When the cpty claims the trade happened
    pub trade_time: DateTime<Utc>,
    #[serde(default)]
    #[pack(default)]
    pub trader: Option<UserId>,
    pub fee: Option<Fee>,
}

#[cfg(feature = "netidx")]
impl Fill {
    pub fn into_aberrant(self) -> AberrantFill {
        AberrantFill {
            kind: Some(self.kind),
            fill_id: self.fill_id,
            order_id: self.order_id,
            account_id: self.account_id,
            market: Some(self.market),
            quantity: Some(self.quantity),
            price: Some(self.price),
            dir: Some(self.dir),
            is_maker: self.is_maker,
            recv_time: self.recv_time,
            trade_time: Some(self.trade_time),
            trader: self.trader,
            fee: self.fee,
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "netidx", derive(Pack))]
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
#[cfg(feature = "netidx")]
#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct AberrantFill {
    pub kind: Option<FillKind>,
    pub fill_id: FillId,
    pub order_id: Option<OrderId>,
    pub account_id: Option<AccountId>,
    pub market: Option<MarketId>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub dir: Option<Dir>,
    pub is_maker: Option<bool>,
    pub recv_time: Option<DateTime<Utc>>,
    pub trade_time: Option<DateTime<Utc>>,
    pub trader: Option<UserId>,
    pub fee: Option<Fee>,
}

#[cfg(feature = "netidx")]
impl AberrantFill {
    /// If sufficient fields on AberrantFill, upgrade it into a Fill
    pub fn try_into_fill(self) -> anyhow::Result<Fill> {
        Ok(Fill {
            kind: self.kind.ok_or_else(|| anyhow!("kind is required"))?,
            fill_id: self.fill_id,
            order_id: self.order_id,
            account_id: self.account_id,
            market: self.market.ok_or_else(|| anyhow!("market is required"))?,
            quantity: self.quantity.ok_or_else(|| anyhow!("quantity is required"))?,
            price: self.price.ok_or_else(|| anyhow!("price is required"))?,
            dir: self.dir.ok_or_else(|| anyhow!("dir is required"))?,
            is_maker: self.is_maker,
            recv_time: self.recv_time,
            trade_time: self
                .trade_time
                .ok_or_else(|| anyhow!("trade_time is required"))?,
            trader: self.trader,
            fee: self.fee,
        })
    }
}
