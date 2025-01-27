//! Accounts represent physical external accounts, mapped by the cpty
//! (only partially user-defined, when the cpty can't disambiguate).
//! There isn't a dichotomy between "internal" and "external" accounts--
//! internal Architect subaccounting should be accomplished via Labels,
//! and account re-labeling or grouping should be done via AccountGroups.
//!
//! If a mislabeling occurs, e.g. use a set of credentials that claim to
//! map to the same account, but don't in actuality, reconciliation
//! errors will be raised by Folio.

use crate::{json_schema_is_string, Str};
use anyhow::{bail, Result};
use derive_more::{Deref, Display};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub type AccountId = Uuid;

#[derive(
    Debug,
    Display,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLScalar))]
#[cfg_attr(feature = "graphql", graphql(transparent))]
pub struct AccountName(Str);

json_schema_is_string!(AccountName);

impl AccountName {
    /// Constructor that codifies some attempt at standard naming convention
    pub fn new(
        venue_name: impl AsRef<str>,
        exchange_account_id: impl AsRef<str>,
    ) -> Result<Self> {
        let name = format!("{}:{}", venue_name.as_ref(), exchange_account_id.as_ref());
        Ok(Self(Str::try_from(name)?))
    }

    pub fn venue_name(&self) -> Option<&str> {
        self.0.split_once(':').map(|(v, _)| v)
    }

    pub fn exchange_account_id(&self) -> Option<&str> {
        self.0.split_once(':').map(|(_, e)| e)
    }
}

impl FromStr for AccountName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Self(Str::try_from(s)?))
        } else {
            bail!("invalid account name: {}", s);
        }
    }
}

#[cfg(feature = "postgres-types")]
impl postgres_types::ToSql for AccountName {
    postgres_types::to_sql_checked!();

    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.as_str().to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        String::accepts(ty)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountIdOrName {
    Id(AccountId),
    Name(AccountName),
}

json_schema_is_string!(AccountIdOrName);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLObject))]
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
}

pub trait AsAccount {
    fn as_account(&self) -> Account;
}

/// Set of flags for account permissions
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    JsonSchema,
)]
pub struct AccountPermissions {
    pub list: bool,            // know about the account's existence
    pub view: bool,            // know the account's holdings and activity
    pub trade: bool,           // trade on the account, any position effect
    pub reduce_or_close: bool, // trade on the account only if reducing or closing
    pub set_limits: bool,      // set limits on the account
}

impl AccountPermissions {
    pub fn all() -> Self {
        Self {
            list: true,
            view: true,
            trade: true,
            reduce_or_close: true,
            set_limits: true,
        }
    }

    pub fn none() -> Self {
        Self {
            list: false,
            view: false,
            trade: false,
            reduce_or_close: false,
            set_limits: false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.list
            && !self.view
            && !self.trade
            && !self.reduce_or_close
            && !self.set_limits
    }

    pub fn list(&self) -> bool {
        self.list
    }

    pub fn view(&self) -> bool {
        self.view
    }

    pub fn trade(&self) -> bool {
        self.trade
    }

    pub fn reduce_or_close(&self) -> bool {
        self.reduce_or_close
    }

    pub fn set_limits(&self) -> bool {
        self.set_limits
    }

    pub fn display(&self) -> String {
        let mut allowed = vec![];
        let mut denied = vec![];
        macro_rules! sift {
            ($perm:ident) => {
                if self.$perm {
                    allowed.push(stringify!($perm));
                } else {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_id_or_name_json() {
        let id: AccountId = "aa0fc734-0da2-4168-8712-4c0b67f01c59".parse().unwrap();
        let name: AccountName = AccountName::new("COINBASE", "TEST").unwrap();

        // Test AccountId serialization
        let id_spec = AccountIdOrName::Id(id);
        insta::assert_json_snapshot!(id_spec, @r#""aa0fc734-0da2-4168-8712-4c0b67f01c59""#);

        // Test AccountId deserialization
        let id_json = r#""aa0fc734-0da2-4168-8712-4c0b67f01c59""#;
        let id_deserialized: AccountIdOrName = serde_json::from_str(id_json).unwrap();
        assert_eq!(id_spec, id_deserialized);

        // Test name serialization
        let name_spec = AccountIdOrName::Name(name);
        insta::assert_json_snapshot!(name_spec, @r#""COINBASE:TEST""#);

        // Test name deserialization
        let name_json = r#""COINBASE:TEST""#;
        let name_deserialized: AccountIdOrName = serde_json::from_str(name_json).unwrap();
        assert_eq!(name_spec, name_deserialized);
    }
}
