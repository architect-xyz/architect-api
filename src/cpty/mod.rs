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
pub enum CptyRequest {
    Login(CptyLoginRequest),
    Logout(CptyLogoutRequest),
    PlaceOrder(Order),
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
pub enum CptyResponse {
    #[serde(rename = "xs")]
    TradableProducts(BTreeMap<TradableProduct, BTreeMap<ExecutionVenue, ExecutionInfo>>),
    #[serde(rename = "ro")]
    ReconcileOrder(Order),
    #[serde(rename = "oo")]
    ReconcileOpenOrders {
        orders: Vec<Order>,
        snapshot_for_account: Option<AccountIdOrName>,
    },
    #[serde(rename = "as")]
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
