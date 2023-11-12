use anyhow::{bail, Result};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, FromValue, Serialize, Deserialize)]
pub enum Dir {
    #[serde(alias = "Buy", alias = "buy", alias = "BUY")]
    Buy,
    #[serde(alias = "Sell", alias = "sell", alias = "SELL")]
    Sell,
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
