//! A tradable product describes a trading pair, e.g. BTC/USD.
//!
//! For products whose quote currency is unambiguous, it may be omitted.

use super::Product;
use anyhow::{bail, Result};
use derive_more::{AsRef, Display};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Debug,
    Display,
    AsRef,
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
#[as_ref(forward)]
#[serde(transparent)]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
pub struct TradableProduct(String);

impl TradableProduct {
    pub fn new(base: &Product, quote: Option<&Product>) -> Result<Self> {
        if base.0.contains('/') {
            bail!("base product cannot contain '/'");
        }
        match quote {
            Some(quote) => {
                if quote.0.contains('/') {
                    bail!("quote product cannot contain '/'");
                }
                Ok(Self(format!("{base}/{quote}")))
            }
            None => Ok(Self(base.0.clone())),
        }
    }

    pub(crate) fn new_unchecked(symbol: impl AsRef<str>) -> Self {
        Self(symbol.as_ref().to_string())
    }

    pub fn base(&self) -> Product {
        match self.0.split_once('/') {
            Some((base, _)) => Product::new_unchecked(base.to_string()),
            None => Product::new_unchecked(self.0.clone()),
        }
    }

    pub fn quote(&self) -> Option<Product> {
        match self.0.split_once('/') {
            Some((_, quote)) => Some(Product::new_unchecked(quote.to_string())),
            None => None,
        }
    }
}

impl std::borrow::Borrow<str> for TradableProduct {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl FromStr for TradableProduct {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().filter(|c| *c == '/').count() > 1 {
            bail!("tradable product symbol cannot contain more than one forward slash character '/'");
        }
        Ok(Self::new_unchecked(s))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct TradableProductInfo {
    pub quote: Product,
}
