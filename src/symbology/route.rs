//! Routes represents a specific path to a venue.  For direct exchange or venue
//! connections, this is represented by the canonical route "DIRECT".  For brokered
//! or third-party connections, e.g. NASDAQ via JPM, this would be represented by
//! the route "JPM".

use crate::{uuid_val, Str};
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

static ROUTE_NS: Uuid = uuid!("0cadbcc5-98bc-4888-94ba-fbbcb6f39132");
uuid_val!(RouteId, ROUTE_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct Route {
    pub id: RouteId,
    pub name: Str,
}
