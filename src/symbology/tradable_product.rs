//! A tradable product describes a trading pair, e.g. BTC/USD.
//!
//! For products whose quote currency is unambiguous, it may be omitted.

use super::Product;
use anyhow::{bail, Result};
use derive_more::{AsRef, Deref, Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Debug,
    Display,
    Deref,
    From,
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
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "juniper", graphql(transparent))]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct TradableProduct(pub String);

impl TradableProduct {
    pub fn new(base: &Product, quote: Option<&Product>) -> Result<Self> {
        if base.contains('/') {
            bail!("base product cannot contain '/'");
        }
        match quote {
            Some(quote) => {
                if quote.contains('/') {
                    bail!("quote product cannot contain '/'");
                }
                Ok(Self(format!("{base}/{quote}")))
            }
            None => Ok(Self(base.to_string())),
        }
    }

    pub fn base(&self) -> Product {
        match self.split_once('/') {
            Some((base, _)) => Product(base.to_string()),
            None => Product(self.0.clone()),
        }
    }

    pub fn quote(&self) -> Option<Product> {
        self.0.split_once('/').map(|(_, quote)| Product(quote.to_string()))
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
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct TradableProductInfo {
    pub quote: Product,
}
