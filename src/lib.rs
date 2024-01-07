pub mod algo;
pub mod config;
pub mod cpty;
pub mod folio;
pub mod marketdata;
pub mod oms;
pub mod orderflow;
pub mod symbology;
pub mod system_control;
pub mod typed_message;
pub mod utils;

// common, basic types which should cover a lot of use cases
pub use config::Config;
pub use orderflow::{ChannelId, OrderId};
pub use typed_message::{MaybeSplit, TypedMessage};
pub use utils::{
    component_id::ComponentId,
    dir::Dir,
    dir_pair::DirPair,
    envelope::{Envelope, RemoteStamp, Stamp},
    half_open_range::HalfOpenRange,
    str::Str,
};
