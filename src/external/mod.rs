//! External plugin wire protocol.  Designed with cross-language,
//! multi-encoding in mind.

use serde::{Deserialize, Serialize};

pub mod marketdata;
pub mod symbology;

// NB: https://github.com/serde-rs/json/issues/545
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessageHeader<'a> {
    pub r#type: &'a str,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "query")]
pub struct ProtocolQueryMessage<T> {
    pub method: String,
    pub id: u64,
    pub params: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "response")]
pub struct ProtocolResponseMessage<T> {
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<ProtocolError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolError {
    pub code: i64,
    pub message: String,
}

// expecting a ProtocolResponseMessage with matching id on success (result=id)/error
// and subsequent ProtocolUpdateMessage with matching ids after success
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "subscribe")]
pub struct ProtocolSubscribeMessage {
    pub id: u64,
    pub topic: String,
    // CR alee: think harder whether to allow subscriptions to be parametrized
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "unsubscribe")]
pub struct ProtocolUnsubscribeMessage {
    pub id: u64,
    pub topic: String,
    pub sub_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "update")]
pub struct ProtocolUpdateMessage<T> {
    pub id: u64,
    pub data: T,
}
