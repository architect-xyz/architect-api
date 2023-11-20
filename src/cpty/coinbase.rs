use crate::{
    orderflow::*,
    symbology::{market::NormalizedMarketInfo, MarketId},
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CoinbaseMarketInfo {
    pub min_market_funds: Decimal,
    pub status_message: Option<String>,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub trading_disabled: bool,
    pub cancel_only: bool,
    pub post_only: bool,
    pub limit_only: bool,
    pub fx_stablecoin: bool,
    pub auction_mode: bool,
}

impl NormalizedMarketInfo for CoinbaseMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.quote_increment
    }

    fn step_size(&self) -> Decimal {
        self.base_increment
    }

    fn is_delisted(&self) -> bool {
        // CR alee: not really true?
        self.trading_disabled
    }
}

#[derive(Debug, Clone, Pack, FromValue)]
pub enum CoinbaseMessage {
    Order(CoinbaseOrder),
    Cancel(Cancel),
    Reject(Reject),
    Ack(Ack),
    Fill(CoinbaseFill),
    Out(Out),
    ExchangeOrderUpdate(OrderId),
    ExchangeAck(OrderId, Uuid),
    ExchangeFills(Vec<CoinbaseFill>),
    ExchangeExternalOrderUpdate(MarketId, Uuid),
    ExchangeExternalOrderNew(Uuid, CoinbaseOrder),
    ExchangeExternalOrderOut(Uuid),
}

impl TryInto<OrderflowMessage> for &CoinbaseMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            CoinbaseMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            CoinbaseMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            CoinbaseMessage::Reject(r) => Ok(OrderflowMessage::Reject(*r)),
            CoinbaseMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            CoinbaseMessage::Fill(f) => Ok(OrderflowMessage::Fill(**f)),
            CoinbaseMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            CoinbaseMessage::ExchangeOrderUpdate(..)
            | CoinbaseMessage::ExchangeAck(..)
            | CoinbaseMessage::ExchangeFills(..)
            | CoinbaseMessage::ExchangeExternalOrderUpdate(..)
            | CoinbaseMessage::ExchangeExternalOrderNew(..)
            | CoinbaseMessage::ExchangeExternalOrderOut(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct CoinbaseOrder {
    #[serde(flatten)]
    pub order: Order,
    #[allow(dead_code)]
    pub special_coinbase_flag: (),
}

impl From<Order> for CoinbaseOrder {
    fn from(order: Order) -> Self {
        Self { order, special_coinbase_flag: () }
    }
}

impl Deref for CoinbaseOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl DerefMut for CoinbaseOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct CoinbaseFill {
    #[serde(flatten)]
    pub fill: Result<Fill, AberrantFill>,
    pub exchange_trade_id: Uuid,
    pub exchange_order_id: Uuid,
}

impl Deref for CoinbaseFill {
    type Target = Result<Fill, AberrantFill>;

    fn deref(&self) -> &Self::Target {
        &self.fill
    }
}

impl std::fmt::Display for CoinbaseMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match &self.status_message {
            Some(status) => status.clone(),
            None => String::from(""),
        };

        write!(
            f,
            "min market funds: {}\n\
                    status message: {}\n\
                    base increment: {}\n\
                    quote increment: {}\n\
                    trading disabled: {}\n\
                    cancel only: {}\n\
                    post only: {}\n\
                    limit only: {}\n\
                    is fx stablecoin: {}\n\
                    auction mode: {}",
            self.min_market_funds,
            status,
            self.base_increment,
            self.quote_increment,
            self.trading_disabled,
            self.cancel_only,
            self.post_only,
            self.limit_only,
            self.fx_stablecoin,
            self.auction_mode
        )
    }
}
