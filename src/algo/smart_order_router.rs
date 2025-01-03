use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage,
    symbology::{MarketId, ProductId},
    Dir, HumanDuration, OrderId,
};
use anyhow::bail;
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type SmartOrderRouterMessage = AlgoContainerMessage<
    SmartOrderRouterOrder,
    NoModification,
    AlgoPreview,
    AlgoStatus,
    AlgoLog,
>;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub struct SmartOrderRouterOrder {
    pub order_id: OrderId,
    pub trader: UserId,
    // These should all be trading markets.
    pub markets: Arc<Vec<MarketId>>,
    pub base: ProductId,
    pub quote: ProductId,
    pub dir: Dir,
    pub limit_price: Decimal,
    pub target_size: Decimal,
    pub execution_time_limit: HumanDuration,
    pub parent_order_id: Option<OrderId>,
}

impl Into<AlgoOrder> for &SmartOrderRouterOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            trader: self.trader,
            account: None,
            algo: AlgoKind::SmartOrderRouter,
            parent_order_id: self.parent_order_id,
            markets: self.markets.clone(),
        }
    }
}

impl Validate for SmartOrderRouterOrder {
    fn validate(&self) -> Result<()> {
        if !self.target_size.is_sign_positive() {
            bail!("target_size must be positive");
        }
        if self.execution_time_limit.num_seconds() <= 0 {
            bail!("execution_time_limit must be positive");
        }
        Ok(())
    }
}
