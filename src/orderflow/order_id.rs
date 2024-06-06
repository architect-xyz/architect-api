use anyhow::{bail, Result};
use base64::Engine;
use bytes::BytesMut;
use compact_str::CompactString;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, str::FromStr};

/// System-unique, persistent order identifiers
#[derive(
    Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", pack(unwrapped))]
pub struct OrderId(pub(crate) u64);

impl OrderId {
    /// Can use this for debugging, tests; not recommended for
    /// production, ask for an allocation from OrderAuthority
    /// component and use an OrderIdAllocator instead.
    pub fn new_unchecked(id: u64) -> Self {
        Self(id)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn encode_base64(&self) -> Result<CompactString> {
        let base64 = base64::engine::general_purpose::STANDARD;
        let bytes = self.0.to_be_bytes();
        let mut output_buf: [u8; 12] = [0; 12];
        let size = base64.encode_slice(&bytes, &mut output_buf)?;
        let cs = CompactString::from_utf8_lossy(&output_buf[0..size]);
        Ok(cs)
    }

    pub fn decode_base64(input: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(input)?;
        if bytes.len() != 8 {
            bail!("incorrect number of bytes to decode OrderId from base64");
        }
        let oid = u64::from_be_bytes(bytes.as_slice().try_into().unwrap()); // can't fail
        Ok(Self(oid))
    }
}

impl FromStr for OrderId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str(s)?))
    }
}

impl fmt::Debug for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for OrderId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};
        let val = Value::Integer(self.0 as i64);
        Ok(ToSqlOutput::Owned(val))
    }
}

impl tokio_postgres::types::ToSql for OrderId {
    tokio_postgres::types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut BytesMut,
    ) -> std::result::Result<tokio_postgres::types::IsNull, Box<dyn Error + Sync + Send>>
    {
        (self.0 as i64).to_sql(ty, out)
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        i64::accepts(ty)
    }
}

#[cfg(feature = "juniper")]
impl OrderId {
    fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.0.to_string())
    }

    fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_string_value()
            .map(|s| u64::from_str(s))
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?
            .map(|oid| Self(oid))
            .map_err(|e| e.to_string())
    }

    fn parse_token<S>(value: juniper::ScalarToken<'_>) -> juniper::ParseScalarResult<S>
    where
        S: juniper::ScalarValue,
    {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
