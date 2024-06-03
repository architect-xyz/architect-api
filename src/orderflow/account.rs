//! Accounts represent physical external accounts, mapped by the cpty
//! (only partially user-defined, when the cpty can't disambiguate).
//! There isn't a dichotomy between "internal" and "external" accounts--
//! internal Architect subaccounting should be accomplished via Labels,
//! and account re-labeling or grouping should be done via AccountGroups.
//!
//! If a mislabeling occurs, e.g. use a set of credentials that claim to
//! map to the same account, but don't in actuality, reconciliation
//! errors will be raised by Folio.

use crate::{
    symbology::{Venue, VenueId},
    utils::messaging::MaybeRequest,
    uuid_val, Str, UserId,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use uuid::{uuid, Uuid};

static ACCOUNT_NS: Uuid = uuid!("c9c8b8e8-69f6-4ca2-83b7-76611e5d6d90");
uuid_val!(AccountId, ACCOUNT_NS);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pack, FromValue)]
pub struct Account {
    pub id: AccountId,
    pub name: Str,
    pub venue_id: VenueId,
}

impl Account {
    /// Constructor that codifies some attempt at standard naming convention
    pub fn new(venue: &Venue, exchange_account_id: impl AsRef<str>) -> Result<Self> {
        let name = format!("{}:{}", venue.name, exchange_account_id.as_ref());
        let id = AccountId::from_str(&name)?;
        Ok(Self { id, name: Str::try_from(name.as_str())?, venue_id: venue.id })
    }
}

/// Set of ternary flags for account permissions
///
/// - None = not set (default disallowed)
/// - Some(true) = allowed
/// - Some(false) = disallowed
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Pack, FromValue)]
pub struct AccountPermissions {
    pub list: Option<bool>,  // know about the account's existence
    pub view: Option<bool>,  // know the account's holdings and activity
    pub trade: Option<bool>, // trade on the account, any position effect
    pub reduce_or_close: Option<bool>, // trade on the account only if reducing or closing
    pub set_limits: Option<bool>, // set limits on the account
}

impl AccountPermissions {
    pub fn all() -> Self {
        Self {
            list: Some(true),
            view: Some(true),
            trade: Some(true),
            reduce_or_close: Some(true),
            set_limits: Some(true),
        }
    }

    pub fn none() -> Self {
        Self {
            list: Some(false),
            view: Some(false),
            trade: Some(false),
            reduce_or_close: Some(false),
            set_limits: Some(false),
        }
    }

    /// Truth table for applying default permissions;
    ///
    /// self, default => result
    /// T, * => T
    /// F, * => F
    /// -, d => d
    ///
    /// This is logically equivalent to `Option::or` but I think
    /// this name is potentially less confusing.
    pub fn with_default(&self, default: &Self) -> Self {
        Self {
            list: self.list.or(default.list),
            view: self.view.or(default.view),
            trade: self.trade.or(default.trade),
            reduce_or_close: self.reduce_or_close.or(default.reduce_or_close),
            set_limits: self.set_limits.or(default.set_limits),
        }
    }

    pub fn list(&self) -> bool {
        self.list.unwrap_or(false)
    }

    pub fn view(&self) -> bool {
        self.view.unwrap_or(false)
    }

    pub fn trade(&self) -> bool {
        self.trade.unwrap_or(false)
    }

    pub fn reduce_or_close(&self) -> bool {
        self.reduce_or_close.unwrap_or(false)
    }

    pub fn set_limits(&self) -> bool {
        self.set_limits.unwrap_or(false)
    }

    pub fn display(&self) -> String {
        let mut allowed = vec![];
        let mut denied = vec![];
        macro_rules! sift {
            ($perm:ident) => {
                if self.$perm == Some(true) {
                    allowed.push(stringify!($perm));
                } else if self.$perm == Some(false) {
                    denied.push(stringify!($perm));
                }
            };
        }
        sift!(list);
        sift!(view);
        sift!(trade);
        sift!(reduce_or_close);
        sift!(set_limits);
        format!("allow({}) deny({})", allowed.join(", "), denied.join(", "))
    }
}

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
