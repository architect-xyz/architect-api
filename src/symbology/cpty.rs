use super::{RouteId, VenueId};
use anyhow::bail;
use derive::FromValue;
use netidx_derive::Pack;
use serde_derive::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Pack, FromValue,
)]
pub struct CptyId {
    pub venue: VenueId,
    pub route: RouteId,
}

impl std::fmt::Display for CptyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.venue, self.route)
    }
}

impl std::str::FromStr for CptyId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((vstr, rstr)) = s.split_once('/') {
            let venue = vstr.parse::<VenueId>()?;
            let route = rstr.parse::<RouteId>()?;
            return Ok(Self { venue, route });
        } else {
            bail!("invalid cpty string, expected venue-id/route-id")
        }
    }
}
