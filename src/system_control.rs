use crate::{ComponentId, MessageTopic, UserId};
use bytes::Bytes;
use derive::FromValue;
use enumflags2::BitFlags;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use uuid::Uuid;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum SystemControlMessage {
    Snapshot(SystemSnapshot),
    DebugSnapshot(DebugSystemSnapshot), // for integration testing
    SymbologyReady,
    Shutdown,
    RestartComponent(ComponentId),
    Ping,
    Pong,
    ChannelSubscribe(UserId, u32, BitFlags<MessageTopic>),
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub core_id: Uuid,
    pub last_seqno: u64,
    pub last_remote_seqno: BTreeMap<Uuid, u64>,
    // id => (kind, json cfg, packed state)
    pub components: Arc<BTreeMap<ComponentId, (String, String, Bytes)>>,
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct DebugSystemSnapshot {
    pub core_id: Uuid,
    pub last_seqno: u64,
    pub last_remote_seqno: BTreeMap<Uuid, u64>,
    // id => (kind, json cfg, json state)
    pub components: Arc<BTreeMap<ComponentId, (String, String, String)>>,
}
