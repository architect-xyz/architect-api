use crate::orderflow::OrderflowMessage;
use derive::FromValue;
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Pack, FromValue, JsonSchema)]
pub enum TradingActivityMessage {
    Orderflow(OrderflowMessage),
}

// C.R. jenr24 for alee, why does this have to be a reference?
impl TryFrom<&OrderflowMessage> for TradingActivityMessage {
    type Error = ();

    fn try_from(value: &OrderflowMessage) -> Result<Self, Self::Error> {
        Ok(Self::Orderflow(value.to_owned()))
    }
}
