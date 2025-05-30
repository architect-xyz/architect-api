use crate::{
    folio::{AccountBalances, AccountPositions, AccountStatistics},
    orderflow::{Cancel, Order},
    symbology::{ExecutionInfo, ExecutionVenue, TradableProduct},
    AccountId, AccountIdOrName, UserId,
};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod cpty_id;

pub use cpty_id::CptyId;

#[grpc(package = "json.architect")]
#[grpc(service = "Cpty", name = "cpty", response = "CptyResponse", server_streaming)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t", rename_all = "snake_case")]
/// <!-- py: tag=t -->
pub enum CptyRequest {
    #[schemars(title = "Login|CptyLoginRequest")]
    Login(CptyLoginRequest),
    #[schemars(title = "Logout|CptyLogoutRequest")]
    Logout(CptyLogoutRequest),
    #[schemars(title = "PlaceOrder|Order")]
    PlaceOrder(Order),
    #[schemars(title = "CancelOrder")]
    CancelOrder { cancel: Cancel, original_order: Option<Order> },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptyLoginRequest {
    pub trader: UserId,
    pub account: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptyLogoutRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "t", rename_all = "snake_case")]
/// <!-- py: tag=t -->
pub enum CptyResponse {
    #[serde(rename = "xs")]
    #[schemars(title = "Symbology")]
    Symbology {
        execution_info:
            BTreeMap<TradableProduct, BTreeMap<ExecutionVenue, ExecutionInfo>>,
    },
    #[serde(rename = "ro")]
    #[schemars(title = "ReconcileOrder|Order")]
    ReconcileOrder(Order),
    #[serde(rename = "oo")]
    #[schemars(title = "ReconcileOpenOrders")]
    ReconcileOpenOrders {
        orders: Vec<Order>,
        snapshot_for_account: Option<AccountIdOrName>,
    },
    #[serde(rename = "as")]
    #[schemars(title = "UpdateAccountSummary")]
    UpdateAccountSummary {
        account: AccountIdOrName,
        timestamp: i64,
        timestamp_ns: u32,
        #[serde(default)]
        balances: Option<AccountBalances>,
        #[serde(default)]
        positions: Option<AccountPositions>,
        #[serde(default)]
        statistics: Option<AccountStatistics>,
        is_snapshot: bool,
    },
}

#[grpc(package = "json.architect")]
#[grpc(service = "Cpty", name = "cpty_status", response = "CptyStatus")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptyStatusRequest {
    pub kind: String,
    pub instance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptyStatus {
    pub kind: String,
    pub instance: Option<String>,
    pub connected: bool,
    /// Not applicable to cpty if None
    pub logged_in: Option<bool>,
    pub stale: bool,
    pub connections: BTreeMap<String, ConnectionStatus>,
}

impl CptyStatus {
    pub fn new(id: CptyId) -> Self {
        Self {
            kind: id.kind.to_string(),
            instance: id.instance.map(|s| s.to_string()),
            connected: true,
            logged_in: None,
            stale: false,
            connections: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConnectionStatus {
    pub connected: bool,
    /// Not applicable to connection if None
    pub logged_in: Option<bool>,
    /// UNIX epoch time or -1 for never
    pub last_heartbeat: i64,
    /// Stale threshold in seconds, or -1 for never stale
    pub last_heartbeat_stale_threshold: i64,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Cpty", name = "cptys", response = "CptysResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptysRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CptysResponse {
    pub cptys: Vec<CptyStatus>,
}
