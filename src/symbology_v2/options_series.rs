use super::{DerivativeKind, Product, TradableProduct};
use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    str::{self, FromStr},
    sync::LazyLock,
};

#[derive(
    Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[serde(transparent)]
pub struct OptionsSeries(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionsSeriesInfo {
    pub options_series: OptionsSeries,
    pub venue_discriminant: Option<String>,
    pub quote_symbol: Product,
    pub underlying: Product,
    pub multiplier: Decimal,
    pub strikes: OptionsStrikes,
    pub expiration: OptionsExpirations,
    pub derivative_kind: DerivativeKind,
    pub exercise_type: OptionsExerciseType,
    pub is_cash_settled: bool,
}

impl OptionsSeriesInfo {
    pub fn get_product(&self, instance: &OptionsSeriesInstance) -> Result<Product> {
        let OptionsSeriesInstance { expiration, strike, put_or_call } = instance;
        let stem_and_venue_discriminant = self
            .options_series
            .0
            .strip_suffix(" Options")
            .ok_or_else(|| anyhow!("invalid options series name"))?;
        let stem = if let Some(venue_discriminant) = &self.venue_discriminant {
            stem_and_venue_discriminant
                .strip_suffix(venue_discriminant.as_str())
                .ok_or_else(|| anyhow!("invalid options series name"))?
                .trim_end()
        } else {
            stem_and_venue_discriminant
        };
        Product::option(
            stem,
            expiration.date_naive(),
            *strike,
            *put_or_call,
            self.venue_discriminant.as_deref(),
        )
    }

    pub fn get_tradable_product(
        &self,
        instance: &OptionsSeriesInstance,
    ) -> Result<TradableProduct> {
        let base = self.get_product(instance)?;
        TradableProduct::try_new(&base, Some(&self.quote_symbol))
    }

    pub fn parse_instance(
        &self,
        symbol: impl AsRef<str>,
    ) -> Result<OptionsSeriesInstance> {
        static OPTION_SYMBOL_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(r"^([\w\s]+) (\d{8}) ([\d\.]+) ([PC])( \w*)? Option/?.*$")
                .unwrap()
        });

        // TODO: check stem
        let symbol = symbol.as_ref();
        let caps = OPTION_SYMBOL_RE
            .captures(symbol)
            .ok_or_else(|| anyhow!("symbol does not match expected format"))?;

        let expiration_str = &caps[2];
        let expiration_date = NaiveDate::parse_from_str(expiration_str, "%Y%m%d")?;
        // TODO: check expiration date
        let expiration = expiration_date
            .and_time(self.expiration.time_of_day)
            .and_local_timezone(self.expiration.time_zone)
            .single()
            .ok_or_else(|| anyhow!("expiration time ambiguous with given time zone"))?
            .to_utc();

        let strike = caps[3].parse::<Decimal>()?;
        let put_or_call = caps[4].parse::<PutOrCall>()?;

        // If we expect a venue discriminant, it must exist in the symbol and match
        if let Some(expected_venue) = &self.venue_discriminant {
            let venue_match =
                caps.get(5).ok_or_else(|| anyhow!("missing venue discriminant"))?;
            let venue = venue_match.as_str().trim();
            if venue.is_empty() || venue != expected_venue {
                bail!("venue discriminant mismatch");
            }
        }

        Ok(OptionsSeriesInstance { expiration, strike, put_or_call })
    }
}

/// A specific option from a series.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct OptionsSeriesInstance {
    pub expiration: DateTime<Utc>,
    pub strike: Decimal,
    pub put_or_call: PutOrCall,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct OptionsStrikes {
    pub start: Decimal,
    pub end: Decimal,
    pub stride: Decimal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionsExpirations {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub stride_days: u32,
    pub time_of_day: NaiveTime,
    pub time_zone: chrono_tz::Tz,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionsExerciseType {
    American,
    European,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PutOrCall {
    #[serde(rename = "P")]
    Put,
    #[serde(rename = "C")]
    Call,
}

impl fmt::Display for PutOrCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Put => write!(f, "P"),
            Self::Call => write!(f, "C"),
        }
    }
}

impl FromStr for PutOrCall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "P" => Ok(Self::Put),
            "C" => Ok(Self::Call),
            _ => bail!("invalid PutOrCall: {}", s),
        }
    }
}
