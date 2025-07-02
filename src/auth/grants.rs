use crate::symbology::MarketdataVenue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static ALL: Grants = Grants::all();
pub static NONE: Grants = Grants::none();

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Grants {
    /// If true, auth is scoped to the specific claims rather
    /// than granting broad access permissions;
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketdata: Option<BoolOrList<MarketdataVenue>>,
}

impl Grants {
    pub const fn all() -> Self {
        Self { scoped: Some(false), marketdata: Some(BoolOrList::Bool(true)) }
    }

    pub const fn none() -> Self {
        Self { scoped: Some(true), marketdata: None }
    }

    pub fn is_subset_of(&self, other: &Grants) -> bool {
        if other.scoped.is_none() || other.scoped.is_some_and(|o| o == false) {
            // trivially true
            return true;
        }
        // INVARIANT: other.scoped == Some(true)
        if self.scoped.is_none() || self.scoped.is_some_and(|s| s == false) {
            // trivially false
            return false;
        }
        // INVARIANT: self.scoped == Some(true)
        match (&self.marketdata, &other.marketdata) {
            (None, _) => {}
            (Some(BoolOrList::Bool(false)), _) => {}
            (Some(BoolOrList::List(_)), Some(BoolOrList::Bool(true))) => {}
            (Some(BoolOrList::List(vs)), Some(BoolOrList::List(vs2))) => {
                for v in vs {
                    if !vs2.contains(v) {
                        return false;
                    }
                }
            }
            (Some(BoolOrList::Bool(true)), Some(BoolOrList::Bool(true))) => {}
            _ => {
                return false;
            }
        }
        true
    }

    pub fn is_scoped(&self) -> bool {
        self.scoped.unwrap_or(false)
    }

    pub fn allowed_to_trade(&self) -> bool {
        if self.is_scoped() {
            false
        } else {
            true
        }
    }

    pub fn allowed_to_marketdata(&self, venue: &MarketdataVenue) -> bool {
        if self.is_scoped() {
            if let Some(marketdata) = &self.marketdata {
                marketdata.includes(venue)
            } else {
                false
            }
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum BoolOrList<T> {
    Bool(bool),
    List(Vec<T>),
}

impl<T> BoolOrList<T> {
    pub fn is_all(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::List(_) => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::Bool(b) => !*b,
            Self::List(xs) => xs.is_empty(),
        }
    }
}

impl<T> BoolOrList<T>
where
    T: PartialEq,
{
    pub fn includes(&self, item: &T) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::List(list) => list.contains(item),
        }
    }
}
