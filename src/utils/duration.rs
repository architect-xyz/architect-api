//! Utility functions for working with durations

use crate::json_schema_is_string;
use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Duration, Utc};
use derive_more::{Deref, DerefMut, From};
use serde::{Deserialize, Serialize};
use serde_with::serde_conv;
use std::str::FromStr;

serde_conv!(pub DurationAsStr, Duration, format_duration, parse_duration);

serde_conv!(
    pub NonZeroDurationAsStr,
    std::time::Duration,
    format_nonzero_duration,
    parse_nonzero_duration
);

pub fn format_nonzero_duration(dur: &std::time::Duration) -> String {
    let secs = dur.as_secs();
    let nanos = dur.subsec_nanos();
    format!("{}.{:09}s", secs, nanos)
}

fn parse_nonzero_duration(s: String) -> Result<std::time::Duration> {
    let dur = parse_duration(&s)?;
    if dur.is_zero() {
        bail!("duration must be non-zero");
    }
    Ok(dur.to_std()?)
}

json_schema_is_string!(DurationAsStr);
json_schema_is_string!(NonZeroDurationAsStr);

// CR alee: deprecating in favor of DurationAsStr
#[derive(
    Debug,
    Clone,
    Copy,
    From,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct HumanDuration(
    #[serde(
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub Duration,
);

impl FromStr for HumanDuration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_duration(s).map(HumanDuration)
    }
}

json_schema_is_string!(HumanDuration);

/// Helper struct to parse from either an absolute ISO 8601 datetime,
/// or some duration relative to now (e.g. +1h, -3d, etc.)
#[derive(Debug, Clone)]
pub enum AbsoluteOrRelativeTime {
    Absolute(DateTime<Utc>),
    RelativeFuture(Duration),
    RelativePast(Duration),
    Now,
}

impl AbsoluteOrRelativeTime {
    pub fn resolve_to(&self, now: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Self::Absolute(dt) => *dt,
            Self::RelativeFuture(d) => now + *d,
            Self::RelativePast(d) => now - *d,
            Self::Now => now,
        }
    }

    pub fn resolve(&self) -> DateTime<Utc> {
        self.resolve_to(Utc::now())
    }
}

impl FromStr for AbsoluteOrRelativeTime {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "now" {
            Ok(Self::Now)
        } else if let Some(rest) = s.strip_prefix('+') {
            Ok(Self::RelativeFuture(parse_duration(rest)?))
        } else if s.starts_with('_') || s.starts_with("~") || s.starts_with('-') {
            // CR-someday alee: clap is actually a bad library in a lot of ways, including
            // not understanding a leading '-' in argument value following a flag
            Ok(Self::RelativePast(parse_duration(&s[1..])?))
        } else {
            Ok(Self::Absolute(DateTime::from_str(s)?))
        }
    }
}

// TODO: pick a more elegant format rather than dumb seconds
fn format_duration(dur: &Duration) -> String {
    let secs = dur.num_milliseconds() as f64 / 1000.;
    format!("{}s", secs)
}

/// Parse a duration string into a `chrono::Duration`.
///
/// A valid duration string is an integer or float followed by a
/// suffix. Supported suffixes are,
///
/// - d: days float
/// - h: hours float
/// - m: minutes float
/// - s: seconds float
/// - ms: milliseconds int
/// - us: microseconds int
/// - ns: nanoseconds int
///
/// e.g. 27ns, 1.7d, 22.2233h, 47.3m, ...
pub fn parse_duration(s: &str) -> Result<Duration> {
    if s.ends_with("ns") {
        let s = s.strip_suffix("ns").unwrap().trim();
        let n = s.parse::<i64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::nanoseconds(n))
    } else if s.ends_with("us") {
        let s = s.strip_suffix("us").unwrap().trim();
        let n = s.parse::<i64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::microseconds(n))
    } else if s.ends_with("ms") {
        let s = s.strip_suffix("ms").unwrap().trim();
        let n = s.parse::<i64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::milliseconds(n))
    } else if s.ends_with("s") {
        let s = s.strip_suffix("s").unwrap().trim();
        let f = s.parse::<f64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::nanoseconds((f * 1e9).trunc() as i64))
    } else if s.ends_with("m") {
        let s = s.strip_suffix("m").unwrap().trim();
        let f = s.parse::<f64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::nanoseconds((f * 60. * 1e9).trunc() as i64))
    } else if s.ends_with("h") {
        let s = s.strip_suffix("h").unwrap().trim();
        let f = s.parse::<f64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::nanoseconds((f * 3600. * 1e9).trunc() as i64))
    } else if s.ends_with("d") {
        let s = s.strip_suffix("d").unwrap().trim();
        let f = s.parse::<f64>().map_err(|e| anyhow!(e.to_string()))?;
        Ok(Duration::nanoseconds((f * 86400. * 1e9).trunc() as i64))
    } else {
        Err(anyhow!("expected a suffix ns, us, ms, s, m, h, d"))
    }
}

/// a serde visitor for `chrono::Duration`
pub struct DurationVisitor;

impl serde::de::Visitor<'_> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expecting a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        parse_duration(s).map_err(|e| E::custom(e.to_string()))
    }
}

/// A serde deserialize function for `chrono::Duration`
///
/// using `parse_duration`
pub fn deserialize_duration<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    d.deserialize_str(DurationVisitor)
}

pub fn deserialize_duration_opt<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(d)?;
    match s {
        Some(s) => Ok(Some(parse_duration(&s).map_err(serde::de::Error::custom)?)),
        None => Ok(None),
    }
}

/// A serde serializer function for `chrono::Duration`
///
/// that writes the duration as an f64 number of seconds followed by
/// the s suffix.
pub fn serialize_duration<S>(d: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let secs = d.num_milliseconds() as f64 / 1000.;
    s.serialize_str(&format!("{}s", secs))
}

pub fn serialize_duration_opt<S>(d: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match d {
        Some(d) => {
            let secs = d.num_milliseconds() as f64 / 1000.;
            s.serialize_some(&format!("{}s", secs))
        }
        None => s.serialize_none(),
    }
}
