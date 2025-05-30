use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct ClearingAccount {
    pub account_name: String,
    pub user_email: String,
    pub clearing_firm: String,
    pub firm_account_id: String,
}
