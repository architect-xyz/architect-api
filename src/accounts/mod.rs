use crate::{Account, AccountIdOrName, AccountPermissions, TraderIdOrEmail, UserId};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod account;
pub mod clearing_account;
pub mod trader;

#[grpc(package = "json.architect")]
#[grpc(service = "Accounts", name = "accounts", response = "AccountsResponse")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AccountsRequest {
    /// Request accounts from the perspective of this trader;
    /// if not specified, defaults to the caller user.
    pub trader: Option<TraderIdOrEmail>,
    #[serde(default)]
    pub paper: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AccountsResponse {
    pub accounts: Vec<AccountWithPermissions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct AccountWithPermissions {
    pub account: Account,
    pub trader: UserId,
    pub permissions: AccountPermissions,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Accounts",
    name = "reset_paper_account",
    response = "ResetPaperAccountResponse"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResetPaperAccountRequest {
    /// The trader for whom to reset paper accounts.
    /// If not specified, defaults to the caller user.
    pub account: AccountIdOrName,
    /// Balance to reset paper account to in USD cents
    #[serde(default)]
    pub usd_balance_cents: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResetPaperAccountResponse {}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Accounts",
    name = "open_paper_account",
    response = "OpenPaperAccountResponse"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OpenPaperAccountRequest {
    /// The name for the new paper account (will be of the form "PAPER:{email}:{account_name}")
    /// If not specified, the default account will be created "PAPER:{email}"
    /// Note that you cannot close a paper account once you open it.
    pub account_name: Option<String>,
    /// Balance to open paper account with in USD cents
    pub usd_balance_cents: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OpenPaperAccountResponse {
    /// The newly created paper account
    pub account: Account,
}
