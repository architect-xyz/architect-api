use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
pub enum OptionType {
    #[serde(alias = "Call", alias = "call", alias = "CALL")]
    Call,
    #[serde(alias = "Put", alias = "put", alias = "PUT")]
    Put,
}

#[cfg(feature = "juniper")]
impl OptionType {
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

impl FromStr for OptionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CALL" => Ok(Self::Call),
            "PUT" => Ok(Self::Put),
            _ => Err(anyhow::anyhow!("invalid format: {s}")),
        }
    }
}

impl OptionType {
    pub fn flip(self) -> Self {
        match self {
            Self::Call => Self::Put,
            Self::Put => Self::Call,
        }
    }

    pub fn to_str_uppercase(self) -> &'static str {
        match self {
            Self::Call => "CALL",
            Self::Put => "PUT",
        }
    }

    pub fn from_str_uppercase(s: &str) -> Result<Self> {
        match s {
            "CALL" => Ok(Self::Call),
            "PUT" => Ok(Self::Put),
            _ => bail!("invalid format: {}", s),
        }
    }

    pub fn to_str_lowercase(self) -> &'static str {
        match self {
            Self::Call => "call",
            Self::Put => "put",
        }
    }

    pub fn from_str_lowercase(s: &str) -> Result<Self> {
        match s {
            "call" => Ok(Self::Call),
            "put" => Ok(Self::Put),
            _ => bail!("invalid format: {}", s),
        }
    }
}
