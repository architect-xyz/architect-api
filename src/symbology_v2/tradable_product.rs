//! A tradable product describes a trading pair, e.g. BTC/USD.
//!
//! For products whose quote currency is unambiguous, it may be omitted.

use super::Product;
use anyhow::{bail, Result};
use derive_more::Display;
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
pub struct TradableProduct(String);

impl TradableProduct {
    pub fn try_new(base: &Product, quote: Option<&Product>) -> Result<Self> {
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

    pub fn base(&self) -> Product {
        match self.0.split_once('/') {
            Some((base, _)) => Product::new_unchecked(base),
            None => Product::new_unchecked(&self.0),
        }
    }

    pub fn quote(&self) -> Option<Product> {
        match self.0.split_once('/') {
            Some((_, quote)) => Some(Product::new_unchecked(quote)),
            None => None,
        }
    }
}
