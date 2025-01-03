#![cfg(feature = "netidx")]

use crate::ComponentId;
use bytes::Bytes;
use derive::FromValue;
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use uuid::Uuid;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub enum SystemControlMessage {
    Snapshot(SystemSnapshot),
    DebugSnapshot(DebugSystemSnapshot), // for integration testing
    SymbologyReady,
    Shutdown,
    RestartComponent(ComponentId),
    Ping,
    Pong,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize, JsonSchema)]
pub struct SystemSnapshot {
    pub core_id: Uuid,
    pub last_seqno: u64,
    pub last_remote_seqno: BTreeMap<Uuid, u64>,
    // id => (kind, json cfg, packed state)
    pub components: Arc<BTreeMap<ComponentId, (String, String, Bytes)>>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize, JsonSchema)]
pub struct DebugSystemSnapshot {
    pub core_id: Uuid,
    pub last_seqno: u64,
    pub last_remote_seqno: BTreeMap<Uuid, u64>,
    // id => (kind, json cfg, json state)
    pub components: Arc<BTreeMap<ComponentId, (String, String, String)>>,
}
