use crate::{OrderId, TraderIdOrEmail, UserId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use derive::grpc;
use derive_more::{Display, FromStr};
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;
pub mod builder;
pub mod common_params;
pub mod release_at_time;
pub mod spreader;
pub mod twap;

pub trait Algo {
    const NAME: &'static str;

    type Params: std::fmt::Debug
        + Clone
        + Validate
        + DisplaySymbols
        + Serialize
        + DeserializeOwned
        + JsonSchema
        + Send
        + 'static;

    type Status: std::fmt::Debug
        + Clone
        + Default
        + Serialize
        + DeserializeOwned
        + JsonSchema;
}

pub trait DisplaySymbols {
    fn display_symbols(&self) -> Option<Vec<String>> {
        None
    }
}

impl DisplaySymbols for () {}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "create_algo_order", response = "AlgoOrder")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateAlgoOrderRequest {
    pub algo: String,
    pub id: Option<OrderId>,
    pub parent_id: Option<OrderId>,
    pub trader: Option<TraderIdOrEmail>,
    pub params: Box<RawValue>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "AlgoHelper", name = "_algo_param_types", response = "AlgoParamTypes")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
/// this is used to coerce creation of the params in the schema.json
pub struct AlgoParamTypes {
    pub spreader: (spreader::SpreaderParams, spreader::SpreaderStatus),
}

impl CreateAlgoOrderRequest {
    pub fn builder(algo: impl AsRef<str>) -> builder::CreateAlgoOrderRequestBuilder {
        builder::CreateAlgoOrderRequestBuilder::new(algo)
    }
}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "modify_algo_order", response = "AlgoOrder")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModifyAlgoOrderRequest {
    pub algo_order_id: OrderId,
    pub params: Box<RawValue>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "start_algo", response = "StartAlgoResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartAlgoRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartAlgoResponse {}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "pause_algo", response = "PauseAlgoResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PauseAlgoRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PauseAlgoResponse {}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "stop_algo", response = "StopAlgoResponse")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopAlgoRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopAlgoResponse {}

/// Get generic algo run status
#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "algo_order", response = "AlgoOrder")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrderRequest {
    pub algo_order_id: OrderId,
}

/// Find all algo orders matching the given criteria.
///
/// If limit is not specified, it will default to 100.
#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "algo_orders", response = "AlgoOrdersResponse")]
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrdersRequest {
    pub algo: Option<String>,
    pub parent_order_id: Option<OrderId>,
    pub trader: Option<TraderIdOrEmail>,
    pub display_symbol: Option<String>,
    pub status: Option<Vec<AlgoOrderStatus>>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrdersResponse {
    pub algo_orders: Vec<AlgoOrder>,
}

impl Algo for () {
    const NAME: &'static str = "UNKNOWN";

    type Params = ();
    type Status = ();
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrder {
    pub algo: String,
    pub id: OrderId,
    pub parent_id: Option<OrderId>,
    pub create_time: DateTime<Utc>,
    /// If the algo order is stopped, the time at which it was stopped.
    pub finish_time: Option<DateTime<Utc>>,
    /// If the algo order is stopped, whether the stop was successful.
    pub finish_success: Option<bool>,
    pub status: AlgoOrderStatus,
    pub status_details: Box<RawValue>,
    /// If algo order status is rejected, contains the reject reason;
    /// for algo orders that finished unsuccessfully, contains the error reason.
    pub reject_or_error_reason: Option<String>,
    pub display_symbols: Option<Vec<String>>,
    pub trader: UserId,
    pub params: Box<RawValue>,
    /// Progress of the algo, 0.0 to 1.0, if computable
    pub working_progress: Option<f64>,
    pub num_sent_orders: u32,
    pub num_open_orders: u32,
    pub num_rejects: u32,
    pub num_errors: u32,
}

#[derive(
    Debug,
    Display,
    FromStr,
    FromRepr,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    JsonSchema_repr,
)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum AlgoOrderStatus {
    // Pending = 0,
    Working = 1,
    Rejected = 2,
    Paused = 63,
    // Pausing = 64,
    Stopping = 128,
    Stopped = 127, // same as paused but final
}

impl AlgoOrderStatus {
    pub fn is_alive(&self) -> bool {
        matches!(
            self,
            AlgoOrderStatus::Working
                | AlgoOrderStatus::Paused
                | AlgoOrderStatus::Stopping
        )
    }
}

// CR-someday alee: use something more akin to the validator crate
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Validate for () {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoLog {}
