//! An Architect installation is completely specified by its configuration, including
//! the topology of its components and their individual subconfigurations.

use crate::ComponentId;
use anyhow::{bail, Result};
use netidx::{path::Path, subscriber::DesiredAuth};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub netidx_config: Option<PathBuf>,
    #[serde(default)]
    pub publisher_slack: Option<usize>,
    #[serde(default)]
    pub desired_auth: Option<DesiredAuth>,
    #[serde(default)]
    pub bind_config: Option<String>,
    #[serde(default = "Config::default_hosted_base")]
    pub hosted_base: Path,
    #[serde(default = "Config::default_local_base")]
    pub local_base: Path,
    /// Locally run components in the same process, grouped by thread
    #[serde(default)]
    pub local: Vec<HashMap<ComponentId, (String, serde_json::Value)>>,
    /// Remote components elsewhere on the network
    #[serde(default)]
    pub remote: HashMap<ComponentId, Path>,
}

impl Config {
    pub fn default_path() -> Result<PathBuf> {
        match dirs::config_dir() {
            None => bail!("no default config dir could be found"),
            Some(mut path) => {
                path.push("architect");
                path.push("config.yml");
                Ok(path)
            }
        }
    }

    pub fn default_log_path() -> Result<PathBuf> {
        match dirs::config_dir() {
            None => bail!("no default config dir could be found"),
            Some(mut path) => {
                path.push("architect");
                path.push("logs");
                Ok(path)
            }
        }
    }

    fn default_hosted_base() -> Path {
        Path::from("/architect")
    }

    fn default_local_base() -> Path {
        Path::from("/local/architect")
    }

    pub fn find_local_component(
        &self,
        id: ComponentId,
    ) -> Option<&(String, serde_json::Value)> {
        for group in &self.local {
            if let Some(c) = group.get(&id) {
                return Some(c);
            }
        }
        None
    }

    pub fn find_local_component_of_kind(
        &self,
        kind: &str,
    ) -> Option<(ComponentId, (&String, &serde_json::Value))> {
        for group in &self.local {
            for (id, (k, cfg)) in group {
                if k == kind {
                    return Some((*id, (&k, &cfg)));
                }
            }
        }
        None
    }
}
