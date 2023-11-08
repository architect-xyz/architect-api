/* Copyright 2023 Architect Financial Technologies LLC. This is free
 * software released under the GNU Affero Public License version 3. */

//! This is the protocol for sending symbology over the wire between
//! the symbology server and clients, and from the loaders to the
//! symbology server.

use crate::{uuid_from_str, uuid_val};
use bytes::Bytes;
use cficode::CfiCode;
use derive::FromValue;
use enumflags2::{bitflags, BitFlags};
use netidx::chars::Chars;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema, JsonSchema_repr};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::{borrow::Borrow, cell::RefCell, collections::BTreeMap, ops::Deref};
use uuid::Uuid;

mod cficode;

static PRODUCT_NS: Uuid = Uuid::from_bytes([
    187, 37, 167, 167, 166, 28, 72, 90, 172, 41, 29, 227, 105, 166, 160, 67,
]);
uuid_val!(Product);
uuid_from_str!(Product, PRODUCT_NS);

static VENUE_NS: Uuid = Uuid::from_bytes([
    221, 133, 166, 197, 180, 95, 70, 209, 191, 80, 121, 61, 172, 177, 229, 26,
]);
uuid_val!(Venue);
uuid_from_str!(Venue, VENUE_NS);

static ROUTE_NS: Uuid = Uuid::from_bytes([
    12, 173, 188, 197, 152, 188, 72, 136, 148, 186, 251, 188, 182, 243, 145, 50,
]);
uuid_val!(Route);
uuid_from_str!(Route, ROUTE_NS);

static QUOTE_SYMBOL_NS: Uuid = Uuid::from_bytes([
    56, 204, 134, 10, 56, 85, 79, 159, 186, 211, 188, 108, 94, 222, 114, 40,
]);
uuid_val!(QuoteSymbol);
uuid_from_str!(QuoteSymbol, QUOTE_SYMBOL_NS);

static TRADING_SYMBOL_NS: Uuid = Uuid::from_bytes([
    67, 193, 253, 204, 40, 203, 73, 181, 138, 211, 133, 41, 143, 75, 137, 237,
]);
uuid_val!(TradingSymbol);
uuid_from_str!(TradingSymbol, TRADING_SYMBOL_NS);

static TRADABLE_PRODUCT_NS: Uuid = Uuid::from_bytes([
    11, 254, 133, 140, 167, 73, 67, 169, 169, 158, 109, 31, 49, 167, 96, 173,
]);
uuid_val!(TradableProduct);

impl TradableProduct {
    pub fn from_parts(base: &str, quote: &str, venue: &str, route: &str) -> Self {
        thread_local! {
            static BUF: RefCell<String> = RefCell::new(String::new());
        }
        BUF.with(|buf| {
            let mut buf = buf.borrow_mut();
            buf.clear();
            buf.push_str(base);
            buf.push('/');
            buf.push_str(quote);
            buf.push('*');
            buf.push_str(venue);
            buf.push('/');
            buf.push_str(route);
            Self(Uuid::new_v5(&TRADABLE_PRODUCT_NS, buf.as_bytes()))
        })
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Pack, FromValue, JsonSchema,
)]
pub enum OptionDir {
    Put,
    Call,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Pack,
    FromValue,
    JsonSchema,
)]
pub struct TokenInfo {
    pub token_address: Bytes,
    pub token_decimals: u8,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Pack, FromValue,
)]
#[serde(tag = "type", content = "value")]
pub enum ProductClass {
    Coin {
        token_info: BTreeMap<Venue, TokenInfo>,
    },
    Fiat,
    Equity,
    // deprecated
    StableCoin {
        fiat: Product,
        token_info: BTreeMap<Venue, TokenInfo>,
    },
    Future {
        underlying: Product,
        multiplier: Decimal,
    },
    Option {
        underlying: Product,
        multiplier: Decimal,
    },
    Commodity,
    Energy,
    Metal,
    Index,
    #[pack(other)]
    Unknown,
}

impl ProductClass {
    pub fn kind(&self) -> &'static str {
        match self {
            ProductClass::Coin { .. } => "Coin",
            ProductClass::Fiat => "Fiat",
            ProductClass::Equity => "Equity",
            ProductClass::Commodity => "Commodity",
            ProductClass::Energy => "Energy",
            ProductClass::Metal => "Metal",
            ProductClass::StableCoin { .. } => "StableCoin",
            ProductClass::Future { .. } => "Future",
            ProductClass::Option { .. } => "Option",
            ProductClass::Index => "Index",
            ProductClass::Unknown => "Unknown",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Pack,
    FromValue,
)]
pub struct FlushId(Uuid);

impl FlushId {
    pub fn new() -> FlushId {
        FlushId(Uuid::new_v4())
    }
}

#[bitflags]
#[repr(u64)]
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema_repr,
)]
pub enum TradeFlags {
    /// trading is disabled
    Disabled,
    /// new orders may not be sent, but open orders may be canceled
    CancelOnly,
    /// only market maker orders are allowed
    PostOnly,
    /// only limit orders are accepted
    LimitOnly,
    /// margin trading is enabled
    MarginEnabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Pack, FromValue)]
pub struct TradeInfo {
    pub trading_symbol: TradingSymbol,
    pub quote_increment: Decimal,
    pub base_increment: Decimal,
    pub flags: BitFlags<TradeFlags>,
}

impl PartialOrd for TradeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match self.trading_symbol.partial_cmp(&other.trading_symbol)? {
            r @ (Ordering::Greater | Ordering::Less) => Some(r),
            Ordering::Equal => {
                match self.quote_increment.partial_cmp(&other.quote_increment)? {
                    r @ (Ordering::Greater | Ordering::Less) => Some(r),
                    Ordering::Equal => match self
                        .base_increment
                        .partial_cmp(&other.base_increment)?
                    {
                        r @ (Ordering::Greater | Ordering::Less) => Some(r),
                        Ordering::Equal => {
                            match self.flags.bits().partial_cmp(&other.flags.bits())? {
                                r @ (Ordering::Greater | Ordering::Less) => Some(r),
                                Ordering::Equal => Some(Ordering::Equal),
                            }
                        }
                    },
                }
            }
        }
    }
}

impl Ord for TradeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Pack,
    FromValue,
)]
pub struct QuoteInfo {
    pub primary: QuoteSymbol,
    pub implied: Option<QuoteSymbol>,
    pub l2: Option<QuoteSymbol>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Pack, FromValue,
)]
pub enum SymbologyUpdateKind {
    SetPriceInLimitDollars(Product, Decimal),
    AddProduct {
        name: Chars,
        class: ProductClass,
        price_in_limit_dollars: Decimal,
        #[pack(default)]
        cficode: CfiCode,
    },
    RemoveProduct(Product),
    AddTradableProduct {
        base: Product,
        quote: Product,
        venue: Venue,
        route: Route,
        quote_info: Option<QuoteInfo>,
        trade_info: Option<TradeInfo>,
    },
    RemoveTradableProduct(TradableProduct),
    AddRoute(Chars),
    RemoveRoute(Route),
    AddVenue(Chars),
    RemoveVenue(Venue),
    AddQuoteSymbol(Chars),
    RemoveQuoteSymbol(QuoteSymbol),
    AddTradingSymbol(Chars),
    RemoveTradingSymbol(TradingSymbol),
    /// used by loaders to make sure their local database has all the
    /// changes they just pushed.
    Flush(FlushId),
    /// compressed is a zstd compressed packed Pooled<Vec<SymbologyUpdateKind>>
    Snapshot {
        original_length: usize,
        compressed: Bytes,
    },
    #[pack(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Pack, FromValue)]
pub struct SymbologyUpdate {
    pub sequence_number: u64,
    pub kind: SymbologyUpdateKind,
}
