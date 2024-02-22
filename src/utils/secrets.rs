//! Types for working with the secret store

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use zeroize::{Zeroize, Zeroizing};

/// A type that is either a reference to a secret, serialized as
/// a URI string like secrets://<key>, or a plain literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeSecret<T: Zeroize> {
    Secret(String),
    Plain(Zeroizing<T>),
}

impl<T: Zeroize> MaybeSecret<T> {
    pub fn secret<S: AsRef<str>>(key: S) -> Self {
        MaybeSecret::Secret(key.as_ref().to_string())
    }

    pub fn plain(t: T) -> Self {
        MaybeSecret::Plain(Zeroizing::new(t))
    }
}

// Most useful implementations of T for MaybeSecret will require
// a FromStr implementation.  If you don't have one handy, use
// this macro to get a reasonable-ish one using serde_json.
#[macro_export]
macro_rules! from_str_json {
    ($t:ty) => {
        impl std::str::FromStr for $t {
            type Err = serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                serde_json::from_str(s)
            }
        }
    };
}

impl<T: Display + Serialize + Zeroize> Display for MaybeSecret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MaybeSecret::Secret(s) => write!(f, "secrets://{}", s),
            MaybeSecret::Plain(s) => {
                write!(f, "{}", serde_json::to_string(&**s).map_err(|_| std::fmt::Error)?)
            }
        }
    }
}

impl<T: FromStr + DeserializeOwned + Zeroize> FromStr for MaybeSecret<T> {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("secrets://") {
            Ok(MaybeSecret::Secret(s[10..].to_string()))
        } else {
            Ok(MaybeSecret::Plain(Zeroizing::new(serde_json::from_str(s)?)))
        }
    }
}

impl<T: Serialize + Zeroize> Serialize for MaybeSecret<T> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match self {
            MaybeSecret::Secret(s) => ser.serialize_str(&format!("secrets://{}", s)),
            MaybeSecret::Plain(t) => (&*t).serialize(ser),
        }
    }
}

impl<'de, T: DeserializeOwned + FromStr + Zeroize> Deserialize<'de> for MaybeSecret<T> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize)]
        #[serde(untagged)]
        enum Format<T> {
            SecretOrString(String),
            Plain(T),
        }
        match Format::<T>::deserialize(de)? {
            Format::SecretOrString(s) => {
                if s.starts_with("secrets://") {
                    Ok(MaybeSecret::Secret(s[10..].to_string()))
                } else {
                    // using FromStr here is hacky but it works for the
                    // important cases of T = String, &str, etc... at
                    // the cost of requiring FromStr from structs
                    //
                    // if you're looking for some dumb FromStr to use
                    // try the FromStrJson macro in derive
                    //
                    // maybe there's some trick leveraging auto(de)ref
                    // specialization [https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html]
                    // that could help here?
                    Ok(MaybeSecret::Plain(Zeroizing::new(
                        T::from_str(&s)
                            .map_err(|_| serde::de::Error::custom("could not FromStr"))?,
                    )))
                }
            }
            Format::Plain(t) => Ok(MaybeSecret::Plain(Zeroizing::new(t))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zeroize::ZeroizeOnDrop;

    #[test]
    fn test_from_str() {
        let x: MaybeSecret<u64> = "secrets://foo".parse().unwrap();
        assert_eq!(x, MaybeSecret::secret("foo"));
        let y: MaybeSecret<u64> = "42".parse().unwrap();
        assert_eq!(y, MaybeSecret::plain(42u64));
    }

    #[test]
    fn test_serde() {
        let x: MaybeSecret<u64> = MaybeSecret::secret("asdf");
        let y = serde_json::to_string(&x).unwrap();
        let z = serde_json::from_str(&y).unwrap();
        assert_eq!(x, z);
        let x: MaybeSecret<u64> = MaybeSecret::plain(42);
        let y = serde_json::to_string(&x).unwrap();
        let z = serde_json::from_str(&y).unwrap();
        assert_eq!(x, z);
        let x: MaybeSecret<String> = MaybeSecret::plain("hahaha".to_string());
        let y = serde_json::to_string(&x).unwrap();
        let z = serde_json::from_str(&y).unwrap();
        assert_eq!(x, z);
    }

    #[test]
    fn test_serde_yaml() {
        let x: MaybeSecret<u64> = MaybeSecret::secret("asdf");
        let y = serde_yaml::to_string(&x).unwrap();
        let z = serde_yaml::from_str(&y).unwrap();
        assert_eq!(x, z);
        let x: MaybeSecret<u64> = MaybeSecret::plain(42);
        let y = serde_yaml::to_string(&x).unwrap();
        let z = serde_yaml::from_str(&y).unwrap();
        assert_eq!(x, z);
        let x: MaybeSecret<String> = MaybeSecret::plain("hahaha".to_string());
        let y = serde_yaml::to_string(&x).unwrap();
        let z = serde_yaml::from_str(&y).unwrap();
        assert_eq!(x, z);
    }

    #[test]
    fn test_serde_complex() {
        #[derive(
            Debug, PartialEq, Eq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop,
        )]
        struct Foo {
            bar: u64,
            baz: String,
        }
        from_str_json!(Foo);
        let x: MaybeSecret<Foo> =
            MaybeSecret::plain(Foo { bar: 42, baz: "asdf".to_string() });
        let y = serde_json::to_string(&x).unwrap();
        let z = serde_json::from_str(&y).unwrap();
        assert_eq!(x, z);
        let yy = serde_yaml::to_string(&x).unwrap();
        let zz = serde_yaml::from_str(&yy).unwrap();
        assert_eq!(x, zz);
        let x: MaybeSecret<Foo> = MaybeSecret::secret("my_secret_key");
        let y = serde_json::to_string(&x).unwrap();
        let z = serde_json::from_str(&y).unwrap();
        assert_eq!(x, z);
        let yy = serde_yaml::to_string(&x).unwrap();
        let zz = serde_yaml::from_str(&yy).unwrap();
        assert_eq!(x, zz);
    }
}
