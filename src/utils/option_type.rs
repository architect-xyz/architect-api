use anyhow::{bail, Result};
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub enum OptionType {
    #[serde(alias = "Call", alias = "call", alias = "CALL")]
    Call,
    #[serde(alias = "Put", alias = "put", alias = "PUT")]
    Put,
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
    pub fn flip(&self) -> Self {
        match self {
            Self::Call => Self::Put,
            Self::Put => Self::Call,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Call => 'C',
            Self::Put => 'P',
        }
    }

    pub fn from_char(c: char) -> Result<Self> {
        match c {
            'C' => Ok(Self::Call),
            'P' => Ok(Self::Put),
            _ => bail!("invalid option char: {}", c),
        }
    }

    pub fn to_str_uppercase(&self) -> &'static str {
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

    pub fn to_str_lowercase(&self) -> &'static str {
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
