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

    fn type_name() -> &'static str {
        "product"
    }

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
    /// The one guarantee for [underlying] if set is that it can
    /// be used to uniquely identify strips of related futures
    Future {
        underlying: Option<ProductId>,
        multiplier: Option<Decimal>,
        expiration: Option<DateTime<Utc>>,
    },
    FutureSpread {
        same_side_leg: Option<ProductId>,
        opp_side_leg: Option<ProductId>,
    },
    /// The one guarantee for [underlying] if set is that it can
    /// be used to uniquely identify strips of related options
    Option {
        underlying: Option<ProductId>,
        multiplier: Option<Decimal>,
        expiration: Option<DateTime<Utc>>,
    },
    Index,
    Commodity,
    #[pack(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Pack)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLUnion))]
pub enum TokenInfo {
    ERC20(ERC20TokenInfo),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Pack)]
pub struct ERC20TokenInfo {
    // CR alee: don't use bytes, just use the packed ethers type
    pub address: Bytes,
    pub decimals: u8,
}

#[cfg(feature = "juniper")]
#[cfg_attr(feature = "juniper", juniper::graphql_object)]
impl ERC20TokenInfo {
    // CR alee: resolve above CR before implementing address()

    pub fn decimals(&self) -> crate::utils::graphql_scalars::U8 {
        self.decimals.into()
    }
}
