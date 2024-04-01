use crate::symbology::market::NormalizedMarketInfo;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CqgMarketInfo {
    pub deleted: bool,
    pub symbol_id: Option<String>,
    pub cfi_code: Option<String>,
    pub cqg_contract_symbol: Option<String>,
    pub correct_price_scale: Decimal,
    pub tick_size: Decimal,  // uncorrected
    pub tick_value: Decimal, // uncorrected
    pub trade_size_increment: Decimal,
    pub market_data_delay_ms: i64,
}

impl NormalizedMarketInfo for CqgMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.correct_price_scale * self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.trade_size_increment
    }

    fn is_delisted(&self) -> bool {
        self.deleted
    }
}

impl std::fmt::Display for CqgMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
// pub struct CqgOrder {
//     #[serde(flatten)]
//     pub order: Order,
// }
//
// // CR alee for alanham: just an fyi I impl-ed Deref here which makes
// // it possible to abbrevate order.order as (*order) in some places,
// // which I find more aesthetic
// impl Deref for CqgOrder {
//     type Target = Order;
//
//     fn deref(&self) -> &Self::Target {
//         &self.order
//     }
// }
//
// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub enum CqgSpeculationType {
//     Speculation,
//     Arbitrage,
//     Hedge,
// }
//
// impl From<u32> for CqgSpeculationType {
//     fn from(value: u32) -> Self {
//         match value {
//             1 => CqgSpeculationType::Speculation,
//             2 => CqgSpeculationType::Arbitrage,
//             _ => CqgSpeculationType::Hedge,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
// pub struct CqgOpenPosition {
//     pub qty: Decimal,
//     pub price: Decimal,
//     pub speculation_type: CqgSpeculationType,
//     pub is_short: bool,
//     pub account_id: i32,
//     pub market_id: MarketId,
// }
//
// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
// pub struct ExchangeOpenPosition {
//     pub qty: Decimal,
//     pub price: Decimal,
//     pub speculation_type: CqgSpeculationType,
//     pub is_short: bool,
//     pub account_id: i32,
//     pub contract_id: u32,
// }
//
// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
// pub enum CqgOrderType {
//     Market,
//     Limit,
//     Stop,
//     StopLimit,
//     Cross,
//     Invalid,
// }
//
// impl From<u32> for CqgOrderType {
//     fn from(value: u32) -> Self {
//         match value {
//             1 => CqgOrderType::Market,
//             2 => CqgOrderType::Limit,
//             3 => CqgOrderType::Stop,
//             4 => CqgOrderType::StopLimit,
//             5 => CqgOrderType::Cross,
//             _ => CqgOrderType::Invalid,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
// pub enum CqgDuration {
//     Day,
//     GTC,
//     GTD,
//     GTT,
//     FAK,
//     FOK,
//     ATO,
//     ATC,
//     GFA,
//     Invalid,
// }
//
// impl From<u32> for CqgDuration {
//     fn from(value: u32) -> Self {
//         match value {
//             1 => CqgDuration::Day,
//             2 => CqgDuration::GTC,
//             3 => CqgDuration::GTD,
//             4 => CqgDuration::GTT,
//             5 => CqgDuration::FAK,
//             6 => CqgDuration::FOK,
//             7 => CqgDuration::ATO,
//             8 => CqgDuration::ATC,
//             9 => CqgDuration::GFA,
//             _ => CqgDuration::Invalid,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgExchangeFill {
//     pub price: Decimal,
//     pub quantity: Decimal,
//     pub trans_id: u64,
//     pub cl_ord_id: String,
//     pub trade_time: DateTime<Utc>,
//     pub exchange_trade_id: Option<String>,
//     pub is_aggressor: Option<bool>,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgFill {
//     pub fill: Result<Fill, AberrantFill>,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgReject {
//     #[serde(flatten)]
//     pub order_id: String,
//     pub cl_ord_id: String,
//     pub timestamp: DateTime<Utc>,
//     pub reason: Str,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgRequestReject {
//     pub request_id: u32,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgPositionStatus {
//     pub account_id: i32,
//     pub contract_id: u32,
//     pub open_positions: Vec<CqgOpenPosition>,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct ExchangePositionStatus {
//     pub account_id: i32,
//     pub contract_id: u32,
//     pub open_positions: Vec<ExchangeOpenPosition>,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgAck {
//     pub order_id: String,
//     pub cl_ord_id: String,
//     pub timestamp: DateTime<Utc>,
// }
//
// #[derive(Debug, Clone, Pack, Serialize, Deserialize)]
// pub struct CqgOut {
//     #[serde(flatten)]
//     pub order_id: String,
//     pub timestamp: DateTime<Utc>,
// }
//
// #[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
// pub enum CqgMessage {
//     Order(CqgOrder),
//     Cancel(Cancel),
//     Reject(Reject),
//     Ack(Ack),
//     Fill(CqgFill),
//     Out(Out),
//     ExchangeOut(CqgOut),
//     ExchangeAck(CqgAck),
//     ExchangePositionStatus(ExchangePositionStatus),
//     ExchangeFill(CqgExchangeFill),
//     ExchangeReject(CqgReject),
//     MarketMappings(Vec<(MarketId, u32)>),
//     QueryOrderSnapshot,
//     OpenOrderSnapshot(Vec<CqgOrder>),
//     QueryTradeSnapshot(i32),
//     TradeSnapshot(Vec<CqgFill>),
//     QueryPositionStatus(i32),
//     PositionStatusSnapshot(Vec<CqgOpenPosition>),
// }
//
// impl TryInto<OrderflowMessage> for &CqgMessage {
//     type Error = ();
//
//     fn try_into(self) -> Result<OrderflowMessage, ()> {
//         match self {
//             CqgMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
//             CqgMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
//             CqgMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
//             CqgMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
//             CqgMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
//             CqgMessage::Fill(_)
//             | CqgMessage::MarketMappings(_)
//             | CqgMessage::QueryOrderSnapshot
//             | CqgMessage::OpenOrderSnapshot(_)
//             | CqgMessage::QueryTradeSnapshot(_)
//             | CqgMessage::TradeSnapshot(_)
//             | CqgMessage::QueryPositionStatus(_)
//             | CqgMessage::PositionStatusSnapshot(_)
//             | CqgMessage::ExchangeOut(_)
//             | CqgMessage::ExchangeAck(_)
//             | CqgMessage::ExchangePositionStatus(_)
//             | CqgMessage::ExchangeFill(_)
//             | CqgMessage::ExchangeReject(_) => Err(()),
//         }
//     }
// }
//
// impl TryInto<CqgMessage> for &OrderflowMessage {
//     type Error = ();
//
//     fn try_into(self) -> Result<CqgMessage, ()> {
//         match self {
//             OrderflowMessage::Order(o) => Ok(CqgMessage::Order(CqgOrder { order: *o })),
//             OrderflowMessage::Reject(r) => Ok(CqgMessage::Reject(r.clone())),
//             OrderflowMessage::Fill(f) => Ok(CqgMessage::Fill(CqgFill { fill: *f })),
//             OrderflowMessage::Out(o) => Ok(CqgMessage::Out(Out { order_id: o.order_id })),
//             OrderflowMessage::Cancel(c) => {
//                 Ok(CqgMessage::Cancel(Cancel { order_id: c.order_id }))
//             }
//             OrderflowMessage::Ack(a) => Ok(CqgMessage::Ack(Ack { order_id: a.order_id })),
//         }
//     }
// }
