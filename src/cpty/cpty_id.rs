use anyhow::bail;
use ecow::EcoString;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Most of the time there's only one instance of a component, in which
/// case the component kind uniquely identifies it.  If there are multiple,
/// you may specify an instance name to distinguish them.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct CptyId {
    pub kind: EcoString,
    pub instance: Option<EcoString>,
}

impl CptyId {
    pub const fn inline(kind: &'static str, instance: Option<&'static str>) -> Self {
        Self {
            kind: EcoString::inline(kind),
            instance: match instance {
                Some(s) => Some(EcoString::inline(s)),
                None => None,
            },
        }
    }
}

impl std::fmt::Display for CptyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;
        if let Some(instance) = &self.instance {
            write!(f, "({instance})")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for CptyId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('(') {
            Some((kind, instance_and_paren)) => {
                match instance_and_paren.strip_suffix(')') {
                    Some(instance) => {
                        Ok(Self { kind: kind.into(), instance: Some(instance.into()) })
                    }
                    None => bail!("invalid cpty id, missing closing paren: {s}"),
                }
            }
            None => Ok(Self { kind: s.into(), instance: None }),
        }
    }
}
