//! Product are specific assets, liabilities, tokens, etc. things that can be owned,
//! traded, or exchanged.

use super::{Symbolic, VenueId};
use crate::{uuid_val, Str};
use bytes::Bytes;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::{uuid, Uuid};

static PRODUCT_NS: Uuid = uuid!("bb25a7a7-a61c-485a-ac29-1de369a6a043");
uuid_val!(ProductId, PRODUCT_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct Product {
    pub id: ProductId,
    pub name: Str,
    pub kind: ProductKind,
}

impl Symbolic for Product {
    type Id = ProductId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Str {
        self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
#[serde(tag = "type", content = "value")]
pub enum ProductKind {
    Coin {
        token_info: BTreeMap<VenueId, TokenInfo>,
    },
    Fiat,
    Equity,
    Future {
        underlying: ProductId,
        multiplier: Decimal,
    },
    Option {
        underlying: ProductId,
        multiplier: Decimal,
    },
    Commodity,
    Energy,
    Metal,
    Index,
    #[pack(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub enum TokenInfo {
    // TODO: don't use bytes, just use the packed ethers type
    ERC20 { address: Bytes, decimals: u8 },
}
