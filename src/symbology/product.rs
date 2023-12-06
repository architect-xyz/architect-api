//! Product are specific assets, liabilities, tokens, etc. things that can be owned,
//! traded, or exchanged.

use super::{Symbolic, VenueId};
use crate::{uuid_val, Str};
use anyhow::Result;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::{uuid, Uuid};

static PRODUCT_NS: Uuid = uuid!("bb25a7a7-a61c-485a-ac29-1de369a6a043");
uuid_val!(ProductId, PRODUCT_NS);

#[derive(Debug, Clone, Serialize, Deserialize, Pack, FromValue)]
pub struct Product {
    pub id: ProductId,
    pub name: Str,
    pub kind: ProductKind,
}

impl Product {
    pub fn new(name: &str, kind: ProductKind) -> Result<Product> {
        Ok(Product { id: ProductId::from(name), name: Str::try_from(name)?, kind })
    }
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
    Perpetual,
    Future {
        underlying: ProductId,
        multiplier: Decimal,
        expiration: DateTime<Utc>,
    },
    Option {
        underlying: ProductId,
        multiplier: Decimal,
        expiration: DateTime<Utc>,
    },
    Commodity,
    Energy,
    Metal,
    Index,
    #[pack(other)]
    Unknown,
}

impl ProductKind {
    pub fn name(&self) -> &'static str {
        match self {
            ProductKind::Coin { .. } => "Coin",
            ProductKind::Fiat => "Fiat",
            ProductKind::Equity => "Equity",
            ProductKind::Perpetual => "Perpetual",
            ProductKind::Future { .. } => "Future",
            ProductKind::Option { .. } => "Option",
            ProductKind::Commodity => "Commodity",
            ProductKind::Energy => "Energy",
            ProductKind::Metal => "Metal",
            ProductKind::Index => "Index",
            ProductKind::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Pack)]
pub enum TokenInfo {
    // CR alee: don't use bytes, just use the packed ethers type
    ERC20 { address: Bytes, decimals: u8 },
}
