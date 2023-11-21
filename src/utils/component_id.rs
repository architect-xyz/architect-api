use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt, str::FromStr};

/// Components within an Architect installation are uniquely identified by a 16-bit integer
/// in the range `2..=0xffff`.
///
/// The integers 0 and 1 are reserved as special values and MUST NOT be used as component IDs.
///
/// Canonical meanings of special values:
///
/// * `0x0` -- None
/// * `0x1` -- Self/loopback
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Pack, Serialize, Deserialize,
)]
#[pack(unwrapped)]
#[repr(transparent)]
pub struct ComponentId(pub(crate) u16);

impl ComponentId {
    pub fn new(id: u16) -> Result<Self, ComponentIdError> {
        if id <= 1 {
            Err(ComponentIdError::InvalidId)
        } else {
            Ok(Self(id))
        }
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub fn loopback() -> Self {
        Self(1)
    }

    pub fn is_loopback(&self) -> bool {
        self.0 == 1
    }

    pub fn filename(&self) -> String {
        format!("{}", self.0)
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            write!(f, "#none")
        } else if self.is_loopback() {
            write!(f, "#loopback")
        } else {
            write!(f, "#{}", self.0)
        }
    }
}

impl FromStr for ComponentId {
    type Err = ComponentIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            let id = s[1..].parse::<u16>().map_err(|_| ComponentIdError::ParseError)?;
            Self::new(id)
        } else {
            Err(ComponentIdError::ParseError)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComponentIdError {
    InvalidId,
    ParseError,
}

impl fmt::Display for ComponentIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId => {
                write!(f, "invalid component id; must not be 0 or 1")
            }
            Self::ParseError => {
                write!(f, "invalid component id format; must be of the form #<id>")
            }
        }
    }
}

impl StdError for ComponentIdError {}
