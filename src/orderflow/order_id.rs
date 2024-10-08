use anyhow::Result;
#[cfg(feature = "tokio-postgres")]
use bytes::BytesMut;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-postgres")]
use std::error::Error;
use std::{fmt, str::FromStr};
use uuid::Uuid;

/// System-unique, persistent order identifiers
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", pack(unwrapped))]
pub struct OrderId {
    pub seqid: Uuid,
    pub seqno: u64,
}

impl OrderId {
    /// For use in tests and non-effecting operations only!
    /// For production use, use an OrderIdAllocator from the `sdk` crate.
    pub fn nil(seqno: u64) -> Self {
        Self { seqid: Uuid::nil(), seqno }
    }
}

impl FromStr for OrderId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((seqid_s, seqno_s)) => {
                let seqid = Uuid::from_str(seqid_s)?;
                let seqno = u64::from_str(seqno_s)?;
                Ok(Self { seqid, seqno })
            }
            None => Ok(Self { seqid: Uuid::nil(), seqno: u64::from_str(s)? }),
        }
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.seqid.is_nil() {
            write!(f, "{}", self.seqno)
        } else {
            write!(f, "{}:{}", self.seqid, self.seqno)
        }
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for OrderId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};
        let val = Value::Text(self.to_string());
        Ok(ToSqlOutput::Owned(val))
    }
}

#[cfg(feature = "tokio-postgres")]
impl tokio_postgres::types::ToSql for OrderId {
    tokio_postgres::types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut BytesMut,
    ) -> std::result::Result<tokio_postgres::types::IsNull, Box<dyn Error + Sync + Send>>
    {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        String::accepts(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB: sqlx::Database> sqlx::Encode<'a, DB> for OrderId
where
    String: sqlx::Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'a>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let value = self.to_string();
        <String as sqlx::Encode<DB>>::encode(value, buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB: sqlx::Database> sqlx::Decode<'a, DB> for OrderId
where
    &'a str: sqlx::Decode<'a, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'a>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::Decode<DB>>::decode(value)?;
        Ok(value.parse()?)
    }
}

#[cfg(feature = "sqlx")]
impl<DB: sqlx::Database> sqlx::Type<DB> for OrderId
where
    for<'a> &'a str: sqlx::Type<DB>,
{
    fn type_info() -> <DB as sqlx::Database>::TypeInfo {
        <&str as sqlx::Type<DB>>::type_info()
    }
}

#[cfg(feature = "juniper")]
impl OrderId {
    fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.to_string())
    }

    fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_string_value()
            .map(|s| Self::from_str(s))
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?
            .map_err(|e| e.to_string())
    }

    fn parse_token<S>(value: juniper::ScalarToken<'_>) -> juniper::ParseScalarResult<S>
    where
        S: juniper::ScalarValue,
    {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
