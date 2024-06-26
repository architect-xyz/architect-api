#![cfg(feature = "netidx")]

use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt, str::FromStr};

/// Components within an Architect installation are uniquely identified by a 16-bit integer
/// in the range `1..<0xFFFF`.
///
/// The integers 0 and 0xFFFF are reserved as special values and MUST NOT be used as component IDs.
///
/// Canonical meanings of special values:
///
/// * `0x0` -- None/executor/broadcast
/// * `0xFFFF` -- Self/loopback
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pack,
    FromValue,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
#[pack(unwrapped)]
#[repr(transparent)]
pub struct ComponentId(pub(crate) u16);

impl ComponentId {
    pub fn new(id: u16) -> Result<Self, ComponentIdError> {
        if id == 0 || id == u16::MAX {
            Err(ComponentIdError::InvalidId)
        } else {
            Ok(Self(id))
        }
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn loopback() -> Self {
        Self(u16::MAX)
    }

    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        self.0 == u16::MAX
    }

    pub fn filename(&self) -> String {
        format!("{}", self.0)
    }
}

impl TryFrom<u16> for ComponentId {
    type Error = ComponentIdError;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        Self::new(id)
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

// CR alee: JuniperScalarFromStr macro
#[cfg(feature = "juniper")]
impl ComponentId {
    pub fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.to_string())
    }

    pub fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_string_value()
            .map(|s| s.parse::<Self>())
            .ok_or_else(|| format!("Expected `String`, found: {v}"))?
            .map_err(|e| e.to_string())
    }

    pub fn parse_token<S>(
        value: juniper::ScalarToken<'_>,
    ) -> juniper::ParseScalarResult<S>
    where
        S: juniper::ScalarValue,
    {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
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
                write!(f, "invalid component id; must not be 0 or 0xFFFF")
            }
            Self::ParseError => {
                write!(f, "invalid component id format; must be of the form #<id>")
            }
        }
    }
}

impl StdError for ComponentIdError {}
