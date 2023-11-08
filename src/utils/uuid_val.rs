/// Implement a named wrapper type around a UUID
#[macro_export]
macro_rules! uuid_val {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Hash,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Serialize,
            Deserialize,
            Pack,
            FromValue,
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
                Ok(Self(s.parse::<uuid::Uuid>()?))
            }
        }

        impl Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Borrow<uuid::Uuid> for $name {
            fn borrow(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl JsonSchema for $name {
            fn schema_name() -> String {
                format!("{}Id", stringify!($name)).to_string()
            }

            fn json_schema(gen: &mut SchemaGenerator) -> Schema {
                uuid::Uuid::json_schema(gen)
            }
        }
    };
}

/// Implement From<AsRef<str>> for a UUIDv5, using a given namespace
#[macro_export]
macro_rules! uuid_from_str {
    ($t:ident, $ns:ident) => {
        impl<S: AsRef<str>> From<S> for $t {
            fn from(s: S) -> Self {
                Self(Uuid::new_v5(&$ns, s.as_ref().as_bytes()))
            }
        }
    };
}
