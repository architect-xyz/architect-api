use crate::{Account, AccountPermissions, TraderIdOrEmail, UserId};
use derive::grpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod account;
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
