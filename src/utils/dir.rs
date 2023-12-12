use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An order side/direction or a trade execution side/direction.
/// In GraphQL these are serialized as "buy" or "sell".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
pub enum Dir {
    #[serde(alias = "Buy", alias = "buy", alias = "BUY")]
    Buy,
    #[serde(alias = "Sell", alias = "sell", alias = "SELL")]
    Sell,
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
            "BUY" => Ok(Self::Buy),
            "SELL" => Ok(Self::Sell),
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

    pub fn to_str_uppercase(self) -> &'static str {
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

    pub fn to_str_lowercase(self) -> &'static str {
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
}
