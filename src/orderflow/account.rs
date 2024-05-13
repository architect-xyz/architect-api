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
    uuid_val, Str,
};
use anyhow::Result;
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
        let name = format!("{}:{}", venue.name.as_ref(), exchange_account_id.as_ref());
        let id = AccountId::from_str(&name)?;
        Ok(Self { id, venue_id: venue.id, name: Str::try_from(name.as_str())? })
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum AccountMessage {
    MapAccount(Account),
    GetAccounts(Uuid),
    Accounts(Uuid, Arc<Vec<Account>>),
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
            AccountMessage::Accounts(uuid, _) => Some(*uuid),
            _ => None,
        }
    }
}
