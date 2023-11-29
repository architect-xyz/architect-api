use crate::ComponentId;
use bytes::Bytes;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum SystemControlMessage {
    // id => (kind, json cfg, packed state)
    Snapshot(Arc<HashMap<ComponentId, (String, String, Bytes)>>),
    SymbologyReady,
    Shutdown,
}
