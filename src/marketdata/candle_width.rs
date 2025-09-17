use anyhow::anyhow;
use chrono::TimeDelta;
use derive_more::Display;
use schemars::JsonSchema_repr;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumIter,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
#[repr(i32)]
pub enum CandleWidth {
    OneSecond = 1,
    FiveSecond = 5,
    OneMinute = 60,
    TwoMinute = 120,
    ThreeMinute = 180,
    FifteenMinute = 900,
    OneHour = 3600,
    OneDay = 86400,
}

impl CandleWidth {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OneSecond,
            Self::FiveSecond,
            Self::OneMinute,
            Self::TwoMinute,
            Self::ThreeMinute,
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
            Self::TwoMinute => "2m",
            Self::ThreeMinute => "3m",
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
            Self::TwoMinute => 120,
            Self::ThreeMinute => 180,
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
            "2m" => Ok(Self::TwoMinute),
            "3m" => Ok(Self::ThreeMinute),
            "15m" => Ok(Self::FifteenMinute),
            "1h" => Ok(Self::OneHour),
            "1d" => Ok(Self::OneDay),
            _ => Err(anyhow!("invalid candle width: {}", s)),
        }
    }
}

impl From<CandleWidth> for TimeDelta {
    fn from(val: CandleWidth) -> Self {
        TimeDelta::seconds(val.as_seconds())
    }
}
