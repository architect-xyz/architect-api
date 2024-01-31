use super::*;
use crate::{
    algo::generic_container::AlgoContainerMessage,
    symbology::{MarketId, ProductId},
    Dir, HumanDuration, OrderId,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type SmartOrderRouterMessage =
    AlgoContainerMessage<SmartOrderRouterOrder, AlgoPreview, AlgoStatus, AlgoLog>;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct SmartOrderRouterOrder {
    pub order_id: OrderId,
    pub markets: Arc<Vec<MarketId>>,
    pub base: ProductId,
    pub quote: ProductId,
    pub dir: Dir,
    pub limit_price: Decimal,
    pub target_size: Decimal,
    pub execution_time_limit: HumanDuration,
}

impl Into<AlgoOrder> for &SmartOrderRouterOrder {
    fn into(self) -> AlgoOrder {
        AlgoOrder {
            order_id: self.order_id,
            algo: Str::try_from("SMART-ORDER-ROUTER").unwrap(), // won't panic
        }
    }
}
