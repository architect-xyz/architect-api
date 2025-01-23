use anyhow::anyhow;
use chrono::TimeDelta;
use enumflags2::bitflags;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[bitflags]
#[repr(u8)]
pub enum CandleWidth {
    OneSecond,
    FiveSecond,
    OneMinute,
    FifteenMinute,
    OneHour,
    OneDay,
}

impl CandleWidth {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OneSecond,
            Self::FiveSecond,
            Self::OneMinute,
            Self::FifteenMinute,
            Self::OneHour,
            Self::OneDay,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneSecond => "1s",
            Self::FiveSecond => "5s",
            Self::OneMinute => "1m",
            Self::FifteenMinute => "15m",
            Self::OneHour => "1h",
            Self::OneDay => "1d",
        }
    }

    pub fn as_seconds(&self) -> i64 {
        match self {
            Self::OneSecond => 1,
            Self::FiveSecond => 5,
            Self::OneMinute => 60,
            Self::FifteenMinute => 900,
            Self::OneHour => 3600,
            Self::OneDay => 86400,
        }
    }
}

impl FromStr for CandleWidth {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1s" => Ok(Self::OneSecond),
            "5s" => Ok(Self::FiveSecond),
            "1m" => Ok(Self::OneMinute),
            "15m" => Ok(Self::FifteenMinute),
            "1h" => Ok(Self::OneHour),
            "1d" => Ok(Self::OneDay),
            _ => Err(anyhow!("invalid candle width: {}", s)),
        }
    }
}

impl Into<TimeDelta> for CandleWidth {
    fn into(self) -> TimeDelta {
        TimeDelta::seconds(self.as_seconds())
    }
}
