use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{ffi::OsStr, fmt::Display, path::PathBuf};

/// A type that is either a file containing the value or the value itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeFile<T> {
    Value(T),
    File(PathBuf),
}

impl<T: Display> Display for MaybeFile<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MaybeFile::Value(v) => write!(f, "{}", v),
            MaybeFile::File(p) => write!(f, "{}", p.display()),
        }
    }
}

impl<T: Clone + DeserializeOwned> MaybeFile<T> {
    pub async fn load(&self) -> Result<T> {
        match self {
            MaybeFile::Value(v) => Ok(v.clone()),
            MaybeFile::File(p) => {
                let contents = tokio::fs::read_to_string(p).await?;
                let t = match p.extension().and_then(OsStr::to_str) {
                    Some("json") => serde_json::from_str(&contents)?,
                    Some("yml") | Some("yaml") => serde_yaml::from_str(&contents)?,
                    Some(other) => bail!("unknown file extension: {}", other),
                    None => bail!("no file extension"),
                };
                Ok(t)
            }
        }
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for MaybeFile<T> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize)]
        #[serde(untagged)]
        enum Format<T> {
            Value(T),
            File(String),
        }
        match Format::<T>::deserialize(de)? {
            Format::Value(t) => Ok(MaybeFile::Value(t)),
            Format::File(s) => Ok(MaybeFile::File(PathBuf::from(s))),
        }
    }
}
