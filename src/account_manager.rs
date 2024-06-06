#![cfg(feature = "netidx")]

use crate::{
    utils::messaging::MaybeRequest, Account, AccountId, AccountPermissions, UserId,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum AccountMessage {
    MapAccounts(Arc<Vec<Account>>),
    SetAccountDefaultPermissions(Arc<Vec<(UserId, AccountId, AccountPermissions)>>),
    SetAccountPermissions(Arc<Vec<(UserId, AccountId, AccountPermissions)>>),
    GetAccounts(Uuid),
    Accounts(Option<Uuid>, Arc<Vec<Account>>),
}

impl MaybeRequest for AccountMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            AccountMessage::GetAccounts(uuid) => Some(*uuid),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            AccountMessage::Accounts(uuid, _) => *uuid,
            _ => None,
        }
    }
}

/// Account manager netidx subscription wire type
#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub struct AccountsUpdate {
    pub epoch: DateTime<Utc>,
    pub sequence_number: u64,
    pub is_snapshot: bool,
    pub accounts: Option<Vec<Account>>,
    pub default_permissions: Option<Vec<(UserId, AccountId, AccountPermissions)>>,
    pub permissions: Option<Vec<(UserId, AccountId, AccountPermissions)>>,
}
