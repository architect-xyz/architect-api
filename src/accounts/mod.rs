use crate::{
    Account, AccountId, AccountIdOrName, AccountPermissions, TraderIdOrEmail, UserId,
};
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
    pub balance: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResetPaperAccountResponse {
    pub success: bool,
    /// Error message if the operation failed
    pub error: Option<String>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Accounts",
    name = "open_paper_account",
    response = "OpenPaperAccountResponse"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OpenPaperAccountRequest {
    /// The name for the new paper account (will be prefixed with PAPER:{email}:)
    pub account_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OpenPaperAccountResponse {
    /// The ID of the newly created paper account (None if failed)
    pub account_id: Option<AccountId>,
    pub success: bool,
    /// Error message if the operation failed
    pub error: Option<String>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Accounts",
    name = "close_paper_account",
    response = "ClosePaperAccountResponse"
)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ClosePaperAccountRequest {
    /// The account to close (by ID or name)
    pub account: AccountIdOrName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ClosePaperAccountResponse {
    pub success: bool,
    /// Error message if the operation failed
    pub error: Option<String>,
}
