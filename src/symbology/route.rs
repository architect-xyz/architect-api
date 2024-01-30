//! Routes represents a specific path to a venue.  For direct exchange or venue
//! connections, this is represented by the canonical route "DIRECT".  For brokered
//! or third-party connections, e.g. NASDAQ via JPM, this would be represented by
//! the route "JPM".

use super::Symbolic;
use crate::{uuid_val, Str};
use anyhow::Result;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

static ROUTE_NS: Uuid = uuid!("0cadbcc5-98bc-4888-94ba-fbbcb6f39132");
uuid_val!(RouteId, ROUTE_NS);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pack, FromValue)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct Route {
    pub id: RouteId,
    pub name: Str,
}

impl Route {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self { id: RouteId::from(name), name: Str::try_from(name)? })
    }
}

impl Symbolic for Route {
    type Id = RouteId;

    fn type_name() -> &'static str {
        "route"
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Str {
        self.name
    }
}
