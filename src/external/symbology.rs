pub use crate::symbology::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbologySnapshot {
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub routes: Vec<Route>,
    pub venues: Vec<Venue>,
    pub products: Vec<Product>,
    pub markets: Vec<Market>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbologyUpdate {
    pub epoch: DateTime<Utc>,
    pub seqno: u64,
    pub routes: Option<Vec<Route>>,
    pub venues: Option<Vec<Venue>>,
    pub products: Option<Vec<Product>>,
    pub markets: Option<Vec<Market>>,
}
