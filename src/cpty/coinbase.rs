use crate::{
    symbology::market::NormalizedMarketInfo, Ack, Dir, Fill, Order, OrderflowMessage,
    Out, Reject,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
}

// maybe we should also handle generic Order/Ack/Fill types
// but you could opt into the more specific one to use the more
// advanced exchange-specific order types
//
// a little From/Into could help
//
// another loose end: choice between restating the cases vs embeding the type,
// programmer should choose former. exercise why left for reader
#[derive(Debug, Clone, Pack, FromValue)]
pub enum CoinbaseMessage {
    CoinbaseOrder(CoinbaseOrder),
    Order(Order),
    Reject(Reject),
    Ack(Ack),
    Fill(Fill),
    Out(Out),
    ExchangeOrderUpdate(u64),
}

impl TryFrom<OrderflowMessage> for CoinbaseMessage {
    type Error = ();

    fn try_from(msg: OrderflowMessage) -> Result<Self, Self::Error> {
        match msg {
            OrderflowMessage::Order(o) => Ok(Self::Order(o)),
            OrderflowMessage::Reject(r) => Ok(Self::Reject(r)),
            OrderflowMessage::Ack(a) => Ok(Self::Ack(a)),
            OrderflowMessage::Fill(f) => Ok(Self::Fill(f)),
            OrderflowMessage::Out(o) => Ok(Self::Out(o)),
        }
    }
}

impl TryInto<OrderflowMessage> for CoinbaseMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, Self::Error> {
        match self {
            Self::CoinbaseOrder(o) => Ok(OrderflowMessage::Order(o.into())),
            Self::Order(o) => Ok(OrderflowMessage::Order(o)),
            Self::Reject(r) => Ok(OrderflowMessage::Reject(r)),
            Self::Ack(a) => Ok(OrderflowMessage::Ack(a)),
            Self::Fill(f) => Ok(OrderflowMessage::Fill(f)),
            Self::Out(o) => Ok(OrderflowMessage::Out(o)),
            Self::ExchangeOrderUpdate(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Pack)]
pub struct CoinbaseOrder {
    pub id: u64,
    pub target: String,
    pub dir: Dir,
    pub price: Decimal,
    pub quantity: Decimal,
    #[allow(dead_code)]
    pub special_coinbase_flag: (),
}

impl From<Order> for CoinbaseOrder {
    fn from(o: Order) -> Self {
        Self {
            id: o.id,
            target: o.target,
            dir: o.dir,
            price: o.price,
            quantity: o.quantity,
            special_coinbase_flag: (),
        }
    }
}

impl Into<Order> for CoinbaseOrder {
    fn into(self) -> Order {
        Order {
            id: self.id,
            target: self.target,
            dir: self.dir,
            price: self.price,
            quantity: self.quantity,
        }
    }
}
