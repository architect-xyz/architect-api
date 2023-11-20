/// Implement a named wrapper type around a UUID
#[macro_export]
macro_rules! uuid_val {
    ($name:ident, $ns:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Hash,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
            netidx_derive::Pack,
            derive::FromValue,
        )]
        pub struct $name(pub uuid::Uuid);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s.parse::<uuid::Uuid>() {
                    Ok(uuid) => Ok(Self(uuid)),
                    Err(_) => Ok(Self::from(s)),
                }
            }
        }

        /// Implement From<AsRef<str>> for a UUIDv5, using a given namespace
        impl<S: AsRef<str>> From<S> for $name {
            fn from(s: S) -> Self {
                Self(Uuid::new_v5(&$ns, s.as_ref().as_bytes()))
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::borrow::Borrow<uuid::Uuid> for $name {
            fn borrow(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl schemars::JsonSchema for $name {
            fn schema_name() -> String {
                format!("{}", stringify!($name)).to_string()
            }

            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                uuid::Uuid::json_schema(gen)
            }
        }
    };
}
