//! A product is a thing you can have a position in.

use super::PutOrCall;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use derive_more::Display;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Display,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    JsonSchema,
)]
#[serde(transparent)]
pub struct Product(pub(crate) String);

impl Product {
    fn try_new(
        name: &str,
        venue_discriminant: Option<&str>,
        product_kind: &str,
    ) -> Result<Self> {
        if name.contains('/')
            || venue_discriminant.map_or(false, |v| v.contains('/'))
            || product_kind.contains('/')
        {
            bail!("Product name cannot contain the forward slash character '/'");
        }
        let inner = match venue_discriminant {
            Some(venue_discriminant) => {
                if venue_discriminant.is_empty() {
                    bail!("Venue discriminant cannot be empty if provided");
                }
                format!("{} {} {}", name, venue_discriminant.to_uppercase(), product_kind)
            }
            None => format!("{} {}", name, product_kind),
        };
        Ok(Self(inner))
    }

    pub(crate) fn new_unchecked(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().to_string())
    }

    pub fn fiat(name: &str) -> Result<Self> {
        if name.contains(char::is_whitespace) {
            bail!("Fiat product name cannot contain whitespace");
        }
        if name.contains('/') {
            bail!("Fiat product name cannot contain the forward slash character '/'");
        }
        Ok(Self(name.to_string()))
    }

    pub fn crypto(name: &str) -> Result<Self> {
        Self::try_new(name, None, "Crypto")
    }

    pub fn future(
        name: &str,
        expiration: NaiveDate,
        venue_discriminant: Option<&str>,
    ) -> Result<Self> {
        let name = format!("{name} {}", expiration.format("%Y%m%d"));
        Self::try_new(&name, venue_discriminant, "Future")
    }

    pub fn perpetual(name: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        Self::try_new(name, venue_discriminant, "Perpetual")
    }

    pub fn index(name: &str, venue_discriminant: Option<&str>) -> Result<Self> {
        Self::try_new(name, venue_discriminant, "Index")
    }

    /// E.g. "AAPL US 20241227 300 C Option"
    pub fn option(
        stem: &str,
        expiration: NaiveDate,
        strike: Decimal,
        put_or_call: PutOrCall,
        venue_discriminant: Option<&str>,
    ) -> Result<Self> {
        let name = format!(
            "{stem} {} {} {put_or_call}",
            expiration.format("%Y%m%d"),
            strike.normalize()
        );
        Self::try_new(&name, venue_discriminant, "Option")
    }

    // pub fn is_series(&self) -> bool {
    //     self.0.ends_with("Option") || self.0.ends_with("Event Contract")
    // }
}
