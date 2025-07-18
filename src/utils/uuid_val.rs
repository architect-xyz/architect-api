/// Implement a named wrapper type around a UUID
#[macro_export]
macro_rules! uuid_val {
    ($name:ident, $ns:ident) => {
        uuid_val!($name, $ns, {});
    };
    ($name:ident, $ns:ident, { $($key:expr_2021 => $value:expr_2021),* $(,)? }) => {
        /// Wrapper type around a UUIDv5 for a given namespace.  These types are
        /// parseable from either the UUIDv5 string representation, or from the
        /// name itself, as they are 1-1.
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
            serde_with::DeserializeFromStr,
        )]
        #[cfg_attr(feature = "juniper", derive(juniper::GraphQLScalar))]
        pub struct $name(pub uuid::Uuid);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                $(
                    if s == $key {
                        return Ok($value);
                    }
                )*
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

        #[cfg(feature = "sqlx")]
        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                self.0.encode(buf)
            }
        }

        #[cfg(feature = "sqlx")]
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(
                value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let value: String = sqlx::Decode::<'r, sqlx::Postgres>::decode(value)?;
                Ok($name::from(&value))
            }
        }

        #[cfg(feature = "sqlx")]
        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                Uuid::type_info()
            }
        }

        #[cfg(feature = "juniper")]
        impl $name {
            #[allow(clippy::wrong_self_convention)]
            fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
                juniper::Value::scalar(self.0.to_string())
            }

            fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
            where
                S: juniper::ScalarValue,
            {
                v.as_string_value()
                    .map(|s| <Self as std::str::FromStr>::from_str(s))
                    .ok_or_else(|| format!("Expected `String`, found: {v}"))?
                    .map(|uuid| Self(*uuid))
                    .map_err(|e| e.to_string())
            }

            fn parse_token<S>(
                value: juniper::ScalarToken<'_>,
            ) -> juniper::ParseScalarResult<S>
            where
                S: juniper::ScalarValue,
            {
                <String as juniper::ParseScalarValue<S>>::from_str(value)
            }
        }

        impl schemars::JsonSchema for $name {
            fn schema_name() -> String {
                format!("{}", stringify!($name)).to_string()
            }

            fn json_schema(
                r#gen: &mut schemars::r#gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                uuid::Uuid::json_schema(r#gen)
            }
        }

        #[cfg(feature = "rusqlite")]
        impl rusqlite::ToSql for $name {
            fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                use rusqlite::types::{ToSqlOutput, Value};
                Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
            }
        }

        #[cfg(feature = "tokio-postgres")]
        impl tokio_postgres::types::ToSql for $name {
            tokio_postgres::types::to_sql_checked!();

            fn to_sql(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut bytes::BytesMut,
            ) -> Result<
                tokio_postgres::types::IsNull,
                Box<dyn std::error::Error + Sync + Send>,
            > {
                self.0.to_sql(ty, out)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool {
                Uuid::accepts(ty)
            }
        }

        #[cfg(feature = "tokio-postgres")]
        impl<'a> tokio_postgres::types::FromSql<'a> for $name {
            fn from_sql(
                ty: &tokio_postgres::types::Type,
                raw: &'a [u8],
            ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                Uuid::from_sql(ty, raw).map($name)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool {
                <Uuid as postgres_types::FromSql>::accepts(ty)
            }
        }
    };
}
