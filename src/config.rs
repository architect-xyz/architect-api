//! Common configuration for Architect installs.

use crate::symbology::MarketdataVenue;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use url::Url;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// TLS client identity to present to upstream Architect services;
    /// should point to the Architect license certificate file.
    pub license: Option<PathBuf>,
    /// TLS client identity key; if not specified, the corresponding
    /// private key will be looked for at the same path but with .key
    /// extension.
    pub license_key: Option<PathBuf>,
    /// If set, use a non-default secrets store path.
    #[serde(default)]
    pub secrets: Option<PathBuf>,
    #[serde(default)]
    pub userdb: Option<Url>,
    #[serde(default)]
    pub symbology: Option<Url>,
    #[serde(default)]
    pub marketdata: BTreeMap<MarketdataVenue, Url>,
    /// Connect to the specified Architect core
    #[serde(default)]
    pub core: Option<Url>,
}
