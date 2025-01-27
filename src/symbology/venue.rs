use derive_more::{Deref, Display, From, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A venue that provides marketdata, e.g. COINBASE, DATABENTO, XIGNITE, etc.
#[derive(
    Debug,
    Display,
    Deref,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    FromStr,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[repr(transparent)]
#[from(forward)]
#[deref(forward)]
#[serde(transparent)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "graphql", graphql(transparent))]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct MarketdataVenue(String);

impl MarketdataVenue {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl std::borrow::Borrow<str> for MarketdataVenue {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl PartialEq<ExecutionVenue> for MarketdataVenue {
    fn eq(&self, other: &ExecutionVenue) -> bool {
        self.0 == other.0
    }
}

/// A venue that provides execution, e.g. CME, CBOE, NYSE, etc.
#[derive(
    Debug,
    Display,
    Deref,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    FromStr,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[repr(transparent)]
#[from(forward)]
#[deref(forward)]
#[serde(transparent)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "graphql", graphql(transparent))]
#[cfg_attr(feature = "postgres", derive(postgres_types::ToSql))]
#[cfg_attr(feature = "postgres", postgres(transparent))]
pub struct ExecutionVenue(String);

impl ExecutionVenue {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl std::borrow::Borrow<str> for ExecutionVenue {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl PartialEq<MarketdataVenue> for ExecutionVenue {
    fn eq(&self, other: &MarketdataVenue) -> bool {
        self.0 == other.0
    }
}
