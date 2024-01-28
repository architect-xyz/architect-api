use anyhow::Result;
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// System-unique, persistent order identifiers
#[derive(
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Pack,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[pack(unwrapped)]
pub struct OrderId(u64);

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
