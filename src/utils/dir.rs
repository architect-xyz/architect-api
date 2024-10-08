use anyhow::{bail, Result};
#[cfg(feature = "tokio-postgres")]
use bytes::BytesMut;
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-postgres")]
use std::error::Error;
use std::str::FromStr;

/// An order side/direction or a trade execution side/direction.
/// In GraphQL these are serialized as "buy" or "sell".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub enum Dir {
    #[serde(alias = "Buy", alias = "buy", alias = "BUY")]
    Buy,
    #[serde(alias = "Sell", alias = "sell", alias = "SELL")]
    Sell,
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for Dir {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, ValueRef};
        let value_ref = match self {
            Self::Buy => ValueRef::Text("BUY".as_ref()),
            Self::Sell => ValueRef::Text("SELL".as_ref()),
        };
        Ok(ToSqlOutput::Borrowed(value_ref))
    }
}

#[cfg(feature = "tokio-postgres")]
impl tokio_postgres::types::ToSql for Dir {
    tokio_postgres::types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut BytesMut,
    ) -> std::result::Result<tokio_postgres::types::IsNull, Box<dyn Error + Sync + Send>>
    {
        let value_repr = match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        };
        value_repr.to_sql(ty, out)
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        String::accepts(ty)
    }
}

#[cfg(feature = "juniper")]
impl Dir {
    fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.to_str_lowercase().to_string())
    }

    fn from_input<S>(v: &juniper::InputValue<S>) -> std::result::Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_string_value()
            .map(|s| Self::from_str_lowercase(s))
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

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" | "Buy" | "BUY" => Ok(Self::Buy),
            "sell" | "Sell" | "SELL" => Ok(Self::Sell),
            _ => Err(anyhow::anyhow!("invalid format: {s}")),
        }
    }
}

impl Dir {
    /// flip the direction Buy -> Sell, Sell -> Buy
    pub fn flip(self) -> Self {
        match self {
            Self::Buy => Self::Sell,
            Self::Sell => Self::Buy,
        }
    }

    pub fn to_str_uppercase(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }

    pub fn from_str_uppercase(s: &str) -> Result<Self> {
        match s {
            "BUY" => Ok(Self::Buy),
            "SELL" => Ok(Self::Sell),
            _ => bail!("invalid format: {}", s),
        }
    }

    pub fn to_str_lowercase(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }

    pub fn from_str_lowercase(s: &str) -> Result<Self> {
        match s {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => bail!("invalid format: {}", s),
        }
    }

    pub fn position_sign(&self) -> Decimal {
        match self {
            Self::Buy => dec!(1),
            Self::Sell => dec!(-1),
        }
    }
}
