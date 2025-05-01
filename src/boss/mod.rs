use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod protocol;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, GraphQLObject)]
pub struct Statement {
    pub statement_uuid: Uuid,
    pub account: String,
    pub statement_type: String,
    pub clearing_firm: String,
    pub timestamp: DateTime<Utc>,
    pub filename: String,
}
