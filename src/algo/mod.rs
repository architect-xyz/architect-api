use crate::{AccountId, OrderId, UserId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod twap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateAlgoOrderRequest<T> {
    pub algo_name: String,
    pub algo_order_id: Option<OrderId>,
    pub parent_order_id: Option<OrderId>,
    pub trader: Option<UserId>,
    pub account: Option<AccountId>,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModifyAlgoOrderRequest<T> {
    pub algo_order_id: OrderId,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartAlgoOrder {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopAlgoOrder {
    pub algo_order_id: OrderId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlgoOrder<T, S> {
    pub algo_name: String,
    pub algo_order_id: OrderId,
    pub parent_order_id: Option<OrderId>,
    pub trader: UserId,
    pub account: AccountId,
    pub create_time: DateTime<Utc>,
    pub display_symbols: Option<Vec<String>>,
    pub last_error: Option<String>,
    pub last_error_time: Option<DateTime<Utc>>,
    pub params: T,
    pub status: S,
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
