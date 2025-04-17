//! Ticker info is product metadata

use super::{ExecutionVenue, Product};
use anyhow::{bail, Result};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Loosely normalized information about exchange products; used to
/// seed symbology loaders and populate extended product info;
///
/// Symbology loaders will use catalog fields to augment and/or
/// cross-check any other load source.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct ProductCatalogInfo {
    pub exchange: ExecutionVenue,
    /// Could be anything really
    pub exchange_product: String,
    pub quote_currency: Option<Product>,
    /// For derivatives contracts, the multiplier
    pub multiplier: Option<Decimal>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    /// Short description of the product, suitable for display in a bubble, for example
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub schedule_description: Option<String>,
    pub settle_method: Option<String>,
    pub price_display_format: Option<String>,
    /// URL to more product info
    pub info_url: Option<String>,
    pub cqg_contract_symbol: Option<String>,
}

impl ProductCatalogInfo {
    pub fn product_root(&self) -> Result<&str> {
        match self.exchange_product.split_once(' ') {
            Some((root, _)) => Ok(root),
            None => bail!("no root for symbol: {}", self.exchange_product),
        }
    }
}
