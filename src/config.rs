//! An Architect installation is completely specified by its configuration, including
//! the topology of its components and their individual subconfigurations.

use crate::ComponentId;
use anyhow::{bail, Result};
use netidx::{path::Path, subscriber::DesiredAuth};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Component location--local to the installation, or hosted by Architect
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Location {
    Hosted,
    Local,
}

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
    /// UserDB registration servers
    #[serde(default = "Config::default_registration_servers")]
    pub registration_servers: Vec<String>,
    #[serde(default = "Config::default_hosted_base")]
    pub hosted_base: Path,
    #[serde(default = "Config::default_local_base")]
    pub local_base: Path,
    /// Where to mount this core's RPCs and channel; also identifies this core
    pub core_base: Path,
    /// Use local symbology instead of centralized symbology
    #[serde(default)]
    pub use_local_symbology: bool,
    /// Use legacy marketdata paths; does not support legacy blockchain marketdata;
    /// not all subsystems respect this flag
    #[serde(default)]
    pub use_legacy_marketdata_paths: bool,
    #[serde(default)]
    pub marketdata_location_override: HashMap<String, Location>,
    /// Locally run components in the same process
    #[serde(default)]
    pub local: HashMap<ComponentId, (String, serde_json::Value)>,
    /// Remote components elsewhere on the network
    #[serde(default)]
    pub remote: HashMap<Path, Vec<(ComponentId, String)>>,
    /// Sync with a remote core at the given base path
    #[serde(default)]
    pub rsync: Option<Path>,
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

    fn default_registration_servers() -> Vec<String> {
        vec!["https://54.163.187.179:5999".into(), "https://35.84.43.204:5999".into()]
    }

    fn default_hosted_base() -> Path {
        Path::from("/architect")
    }

    fn default_local_base() -> Path {
        Path::from("/local/architect")
    }
}
