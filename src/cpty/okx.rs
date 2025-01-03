use crate::{
    folio::FolioMessage,
    orderflow::{
        AberrantFill, Ack, Cancel, CancelAll, Fill, Order, OrderflowMessage, Out, Reject,
        RejectReason,
    },
    symbology::market::{MinOrderQuantityUnit, NormalizedMarketInfo},
    Amount, Dir, OrderId,
};
use chrono::{DateTime, Utc};
use compact_str::CompactString;
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc};

const LIVE: CompactString = CompactString::new_inline("live");

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxMarketInfo {
    pub tick_sz: Decimal,
    pub min_sz: Decimal,
    pub state: Option<CompactString>,
}

impl NormalizedMarketInfo for OkxMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_sz
    }

    fn step_size(&self) -> Decimal {
        self.min_sz
    }

    fn min_order_quantity(&self) -> Amount<Decimal, MinOrderQuantityUnit> {
        return Amount::new(self.min_sz, MinOrderQuantityUnit::Base);
    }

    fn is_delisted(&self) -> bool {
        !(self.state == Some(LIVE))
    }
}

impl std::fmt::Display for OkxMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum OkxMessage {
    Order(OkxOrder),
    Cancel(Cancel),
    CancelAll(OkxCancelAll),
    Reject(Reject),
    Ack(OkxAck),
    Fill(Result<Fill, AberrantFill>),
    Out(Out),
    Folio(FolioMessage),
    ExchangeSnapshot(Arc<OkxSnapshot>),
    ExchangeAccountConfig(OkxAccountConfig),
    ExchangeOrderUpdate(OkxExchangeOrderUpdate),
    ExchangeReject(Reject),
}

// Copied from AccountLevel, just making sure we're clear about which types are internal and which are external
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum OkxAccountLevel {
    Simple,
    SingleCurrencyMargin,
    MultiCurrencyMargin,
    PortfolioMargin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum OkxMarginMode {
    Cross,
    Isolated,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub struct OkxAccountConfig {
    pub account_level: OkxAccountLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxOrder {
    #[serde(flatten)]
    pub order: Order,
    pub margin_mode: OkxMarginMode,
}

impl Deref for OkxOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxAck {
    #[serde(flatten)]
    pub ack: Ack,
}

impl Deref for OkxAck {
    type Target = Ack;

    fn deref(&self) -> &Self::Target {
        &self.ack
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxCancelAll {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxExchangeOrderUpdate {
    pub order_id: OrderId,
    pub exchange_order_id: CompactString,
    pub state: OkxExchangeState,
    pub fill: Option<OkxFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxFill {
    pub order_id: OrderId,
    pub exchange_order_id: CompactString,
    pub fill_sz: Option<Decimal>,
    pub fill_px: Option<Decimal>,
    pub fill_time: Option<DateTime<Utc>>,
    pub trade_id: Option<CompactString>,
    pub dir: Dir,
    pub is_maker: Option<bool>,
    pub acc_sz: Decimal,
    pub fee: Option<Decimal>,
    pub fee_ccy: Option<CompactString>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub enum OkxExchangeState {
    Live,
    Rejected(RejectReason),
    Canceled,
    Filled,
    PartiallyFilled,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct OkxSnapshot {
    pub open_order_ids: Vec<OrderId>,
}

impl TryFrom<&OkxMessage> for OrderflowMessage {
    type Error = ();

    fn try_from(value: &OkxMessage) -> Result<Self, Self::Error> {
        match value {
            OkxMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            OkxMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            OkxMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            OkxMessage::Ack(a) => Ok(OrderflowMessage::Ack(**a)),
            OkxMessage::CancelAll(_) => {
                Ok(OrderflowMessage::CancelAll(CancelAll { venue_id: None }))
            }
            OkxMessage::Fill(f) => Ok(OrderflowMessage::Fill(f.clone())),
            OkxMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            OkxMessage::Folio(_)
            | OkxMessage::ExchangeAccountConfig(_)
            | OkxMessage::ExchangeSnapshot(_)
            | OkxMessage::ExchangeReject(_)
            | OkxMessage::ExchangeOrderUpdate(_) => Err(()),
        }
    }
}

impl TryFrom<&OrderflowMessage> for OkxMessage {
    type Error = ();

    fn try_from(value: &OrderflowMessage) -> Result<Self, Self::Error> {
        match value {
            // CR-someday arao: Make this a parameter the OMS can pass to us
            OrderflowMessage::Order(o) => Ok(OkxMessage::Order(OkxOrder {
                order: *o,
                margin_mode: OkxMarginMode::Isolated,
            })),
            OrderflowMessage::Cancel(c) => Ok(OkxMessage::Cancel(*c)),
            OrderflowMessage::CancelAll(_) => Ok(OkxMessage::CancelAll(OkxCancelAll {})),
            OrderflowMessage::Reject(r) => Ok(OkxMessage::Reject(r.clone())),
            OrderflowMessage::Ack(_a) => Err(()),
            OrderflowMessage::Fill(f) => Ok(OkxMessage::Fill(f.clone())),
            OrderflowMessage::Out(o) => Ok(OkxMessage::Out(*o)),
        }
    }
}

impl TryFrom<&OkxMessage> for FolioMessage {
    type Error = ();

    fn try_from(value: &OkxMessage) -> Result<Self, Self::Error> {
        match value {
            OkxMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for OkxMessage {
    type Error = ();

    fn try_from(value: &FolioMessage) -> Result<Self, Self::Error> {
        Ok(OkxMessage::Folio(value.clone()))
    }
}
