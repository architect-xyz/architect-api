//! Common configuration for Architect installs.

use crate::symbology::MarketdataVenue;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
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

impl Config {
    pub fn license(&self) -> Result<Option<PathBuf>> {
        canonicalize(&self.license)
    }

    pub fn license_key(&self) -> Result<Option<PathBuf>> {
        canonicalize(&self.license_key)
    }

    pub fn secrets(&self) -> Result<Option<PathBuf>> {
        canonicalize(&self.secrets)
    }
}

fn canonicalize(path: &Option<impl AsRef<Path>>) -> Result<Option<PathBuf>> {
    if let Some(path) = path {
        let expanded = expand_tilde(path).ok_or_else(|| {
            anyhow!("while expanding tilde: {}", path.as_ref().display())
        })?;
        let canonicalized = std::fs::canonicalize(&expanded)
            .with_context(|| format!("while resolving path: {}", expanded.display()))?;
        Ok(Some(canonicalized))
    } else {
        Ok(None)
    }
}

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}
