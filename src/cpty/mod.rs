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
    #[schemars(title = "TradableProducts")]
    TradableProducts(BTreeMap<TradableProduct, BTreeMap<ExecutionVenue, ExecutionInfo>>),
    #[serde(rename = "ro")]
    #[schemars(title = "ReconcileOrder|Order")]
    ReconcileOrder(Order),
    #[serde(rename = "oo")]
    #[schemars(title = "ReconcileOpenOrder")]
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
