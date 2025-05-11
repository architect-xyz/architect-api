use super::{Algo, CreateAlgoOrderRequest};
use crate::{OrderId, TraderIdOrEmail};
use anyhow::{anyhow, Result};
use serde_json::value::RawValue;

#[derive(Debug, Default)]
pub struct CreateAlgoOrderRequestBuilder {
    pub algo: Option<String>,
    pub id: Option<OrderId>,
    pub parent_id: Option<OrderId>,
    pub trader: Option<TraderIdOrEmail>,
    pub params: Option<Box<RawValue>>,
}

impl CreateAlgoOrderRequestBuilder {
    pub fn new(algo: impl AsRef<str>) -> Self {
        Self { algo: Some(algo.as_ref().to_string()), ..Default::default() }
    }

    pub fn id(&mut self, id: OrderId) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn parent_id(&mut self, parent_id: OrderId) -> &mut Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn trader(&mut self, trader: TraderIdOrEmail) -> &mut Self {
        self.trader = Some(trader);
        self
    }

    pub fn params<A: Algo>(&mut self, params: A::Params) -> Result<&mut Self> {
        let params = serde_json::value::to_raw_value(&params)?;
        self.params = Some(params);
        Ok(self)
    }

    pub fn build(self) -> Result<CreateAlgoOrderRequest> {
        Ok(CreateAlgoOrderRequest {
            algo: self.algo.ok_or_else(|| anyhow!("algo is required"))?,
            id: self.id,
            parent_id: self.parent_id,
            trader: self.trader,
            params: self.params.ok_or_else(|| anyhow!("params are required"))?,
        })
    }
}
