#![cfg(feature = "netidx")]

use crate::{folio::FolioMessage, orderflow::*};
use derive::FromValue;
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, JsonSchema)]
pub enum MockCptyMessage {
    Orderflow(OrderflowMessage),
    Folio(FolioMessage),
}

impl TryInto<OrderflowMessage> for &MockCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            MockCptyMessage::Orderflow(o) => Ok(o.clone()),
            MockCptyMessage::Folio(_) => Err(()),
        }
    }
}

impl Into<MockCptyMessage> for &OrderflowMessage {
    fn into(self) -> MockCptyMessage {
        MockCptyMessage::Orderflow(self.clone())
    }
}

impl TryInto<FolioMessage> for &MockCptyMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            MockCptyMessage::Orderflow(_) => Err(()),
            MockCptyMessage::Folio(f) => Ok(f.clone()),
        }
    }
}

impl Into<MockCptyMessage> for &FolioMessage {
    fn into(self) -> MockCptyMessage {
        MockCptyMessage::Folio(self.clone())
    }
}
