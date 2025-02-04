use crate::{AccountId, OrderId, UserId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod twap;

pub trait Algo {
    type Params: Validate + Serialize + DeserializeOwned + JsonSchema;
    type Status: Serialize + DeserializeOwned + JsonSchema;
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateAlgoOrderRequest<A: Algo> {
    pub algo_name: String,
    pub algo_order_id: Option<OrderId>,
    pub parent_order_id: Option<OrderId>,
    pub trader: Option<UserId>,
    pub account: Option<AccountId>,
    pub params: A::Params,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModifyAlgoOrderRequest<A: Algo> {
    pub algo_order_id: OrderId,
    pub params: A::Params,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartAlgoOrderRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopAlgoOrderRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrderRequest {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrder<A: Algo> {
    pub algo_name: String,
    pub algo_order_id: OrderId,
    pub parent_order_id: Option<OrderId>,
    pub trader: UserId,
    pub account: AccountId,
    pub create_time: DateTime<Utc>,
    pub display_symbols: Option<Vec<String>>,
    pub last_error: Option<String>,
    pub last_error_time: Option<DateTime<Utc>>,
    pub params: A::Params,
    pub status: A::Status,
    pub state: AlgoState,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum AlgoState {
    Pending,
    Running,
    Stopped,
}

// CR-someday alee: use something more akin to the validator crate
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
