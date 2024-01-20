//! Types for working with the secret store

use netidx::pack::Pack;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

/// A type that is either a reference to a secret, serialized as
/// a URI string like secrets://<key>, or a plain literal.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum MaybeSecret<T: Pack + FromStr> {
    Secret(String),
    // no zeroize here--if you're keeping a secret in plaintext config,
    // you've already lost; this is for debugging and test
    Plain(T),
}

impl<T: std::fmt::Display + Pack + FromStr> std::fmt::Display for MaybeSecret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeSecret::Secret(s) => write!(f, "secrets://{}", s),
            MaybeSecret::Plain(s) => write!(f, "{}", s),
        }
    }
}

impl<T: Pack + FromStr> FromStr for MaybeSecret<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("secrets://") {
            Ok(MaybeSecret::Secret(s[10..].to_string()))
        } else {
            Ok(MaybeSecret::Plain(s.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let x: MaybeSecret<u64> = "secrets://foo".parse().unwrap();
        assert_eq!(x, MaybeSecret::Secret("foo".to_string()));
        let y: MaybeSecret<u64> = "42".parse().unwrap();
        assert_eq!(y, MaybeSecret::Plain(42));
    }

    #[test]
    fn test_serialize() {
        let x: MaybeSecret<u64> = MaybeSecret::Secret("asdf".to_string());
        let x2 = serde_json::to_string(&x).unwrap();
        assert_eq!(x2, "\"secrets://asdf\"");
    }
}
