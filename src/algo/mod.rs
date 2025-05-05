use crate::{OrderId, TraderIdOrEmail, UserId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use derive::grpc;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod common_params;
pub mod twap;

pub trait Algo {
    const NAME: &'static str;

    type Params: std::fmt::Debug
        + Clone
        + Validate
        + Serialize
        + DeserializeOwned
        + JsonSchema
        + Send
        + 'static;

    type Status: std::fmt::Debug + Clone + Serialize + DeserializeOwned + JsonSchema;
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateAlgoOrderRequest<A: Algo> {
    pub id: Option<OrderId>,
    pub parent_id: Option<OrderId>,
    pub trader: Option<TraderIdOrEmail>,
    pub params: A::Params,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModifyAlgoOrderRequest<A: Algo> {
    pub algo_order_id: OrderId,
    pub params: A::Params,
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
#[grpc(service = "Algo", name = "algo_order", response = "AlgoOrder<()>")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrderRequest {
    pub algo_order_id: OrderId,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "algo_orders", response = "AlgoOrdersResponse<()>")]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrdersRequest {
    pub parent_order_id: Option<OrderId>,
    pub trader: Option<TraderIdOrEmail>,
    pub display_symbol: Option<String>,
    pub status: Option<Vec<AlgoOrderStatus>>,
    pub from_inclusive: Option<DateTime<Utc>>,
    pub to_exclusive: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrdersResponse<A: Algo> {
    pub algo_orders: Vec<AlgoOrder<A>>,
}

impl Algo for () {
    const NAME: &'static str = "UNKNOWN";

    type Params = ();
    type Status = ();
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrder<A: Algo> {
    pub id: OrderId,
    pub parent_id: Option<OrderId>,
    pub create_time: DateTime<Utc>,
    pub finish_time: Option<DateTime<Utc>>,
    pub status: AlgoOrderStatus,
    pub status_details: A::Status,
    pub reject_reason: Option<String>,
    pub display_symbols: Option<Vec<String>>,
    pub trader: UserId,
    pub params: A::Params,
    // progress of the algo, 0.0 to 1.0, if computable
    pub working_progress: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum AlgoOrderStatus {
    Pending = 0,
    Working = 1,
    Rejected = 2,
    Paused = 63,
    Pausing = 64,
    Stopped = 127, // same as paused but final
    Stopping = 128,
}

impl AlgoOrderStatus {
    pub fn is_alive(&self) -> bool {
        matches!(self, AlgoOrderStatus::Pending | AlgoOrderStatus::Working)
    }
}

// CR-someday alee: use something more akin to the validator crate
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Validate for () {}
