//! An Architect installation is completely specified by its configuration, including
//! the topology of its components and their individual subconfigurations.

use crate::ComponentId;
use anyhow::{bail, Result};
use netidx::{path::Path, subscriber::DesiredAuth};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Component location--local to the installation, or hosted by Architect
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub audit_log: Option<PathBuf>,
    #[serde(default = "Config::default_hosted_base")]
    pub hosted_base: Path,
    #[serde(default = "Config::default_local_base")]
    pub local_base: Path,
    /// Use local symbology instead of centralized symbology
    #[serde(default)]
    pub use_local_symbology: bool,
    /// Use local license server instead of centralized (for debugging)
    #[serde(default)]
    pub use_local_licensedb: bool,
    /// Use local marketdata paths for the specified cptys
    #[serde(default)]
    pub use_local_marketdata: Vec<String>,
    /// Use legacy marketdata paths for the specified cptys; does not support
    /// legacy blockchain marketdata; not all subsystems respect this flag
    #[serde(default)]
    pub use_legacy_marketdata: Vec<String>,
    /// Use legacy marketdata paths for the specified cptys, for historical
    /// marketdata only.  Live marketdata will use new-style paths.
    #[serde(default)]
    pub use_legacy_hist_marketdata: Vec<String>,
    #[serde(default)]
    pub secrets_path_override: Option<String>,
    /// In addition to netidx-based licensedb authentication, restrict users to
    /// the given authorized domain only.  Subdomains of the domain will also
    /// be accepted.
    #[serde(default)]
    pub authorized_domain: String,
    // TODO: move this to core command-line argument?
    /// Where to mount this core's RPCs and channel; also identifies this core
    pub core_base: Path,
    // TODO: these only make sense for core, API config should have some other way
    // for that matter, who has a consistent view of topology?
    //
    // gateways should do some ComponentID translation, at a minimum
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

    fn default_hosted_base() -> Path {
        Path::from("/architect")
    }

    fn default_local_base() -> Path {
        Path::from("/local/architect")
    }
}
