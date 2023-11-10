//! Venues represent where products can be traded or custodied, e.g. an exchange, an ATS,
//! custodian, blockchain, or DeFi app.

use super::Symbolic;
use crate::{uuid_val, Str};
use anyhow::Result;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

static VENUE_NS: Uuid = uuid!("dd85a6c5-b45f-46d1-bf50-793dacb1e51a");
uuid_val!(VenueId, VENUE_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub struct Venue {
    pub id: VenueId,
    pub name: Str,
    // TODO: maybe VenueInfo
}

impl Venue {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Venue { id: VenueId::from(name), name: Str::try_from(name)? })
    }
}

impl Symbolic for Venue {
    type Id = VenueId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Str {
        self.name
    }
}
