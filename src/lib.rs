pub mod account_manager;
pub mod algo;
pub mod auth;
pub mod channel_control;
pub mod config;
pub mod cpty;
pub mod external;
pub mod folio;
pub mod marketdata;
pub mod misc;
pub mod oms;
pub mod orderflow;
pub mod symbology;
pub mod system_control;
pub mod typed_message;
pub mod utils;

// common, basic types which should cover a lot of use cases
pub use auth::user_id::UserId;
#[cfg(feature = "netidx")]
pub use config::Config;
pub use orderflow::OrderId;
#[cfg(feature = "netidx")]
pub use typed_message::{MaybeSplit, MessageTopic, TypedMessage};
pub use utils::{
    account::{Account, AccountId, AccountPermissions},
    amount::Amount,
    dir::Dir,
    dir_pair::DirPair,
    duration::HumanDuration,
    half_open_range::HalfOpenRange,
    str::Str,
};
#[cfg(feature = "netidx")]
pub use utils::{
    component_id::ComponentId,
    envelope::{Address, Envelope, Sequence, Stamp},
    maybe_file::MaybeFile,
    secrets::MaybeSecret,
};
