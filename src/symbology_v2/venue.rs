use derive_more::Display;
use serde::{Deserialize, Serialize};

/// A venue that provides marketdata, e.g. COINBASE, DATABENTO, XIGNITE, etc.
#[derive(
    Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct MarketdataVenue(String);

impl MarketdataVenue {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl PartialEq<ExecutionVenue> for MarketdataVenue {
    fn eq(&self, other: &ExecutionVenue) -> bool {
        self.0 == other.0
    }
}

/// A venue that provides execution, e.g. CME, CBOE, NYSE, etc.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionVenue(String);

impl ExecutionVenue {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl PartialEq<MarketdataVenue> for ExecutionVenue {
    fn eq(&self, other: &MarketdataVenue) -> bool {
        self.0 == other.0
    }
}
