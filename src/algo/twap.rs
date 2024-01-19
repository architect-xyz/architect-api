use crate::{
    orderflow::{algo::*, OrderIdAllocation, OrderflowMessage},
    symbology::MarketId,
    Dir, DirPair, OrderId, Str,
};
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::Duration;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum TwapMessage {
    AlgoOrder(TwapOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(TwapStatus),
    TwapWakeup(TwapWakeup),
    Orderflow(OrderflowMessage),
    ChildAck(ChildAck),
    ChildOut(ChildOut),
    ChildFill(ChildFill),
    ChildReject(ChildReject),
    OrderIdAllocation(OrderIdAllocation),
    BookUpdate(BookUpdate),
    GetTwapStatuses(Uuid),
    GetTwapStatusesResponse(Uuid, Vec<TwapStatus>),
}

impl TryInto<AlgoMessage> for &TwapMessage {
    type Error = ();

    fn try_into(self) -> Result<AlgoMessage, ()> {
        match self {
            TwapMessage::AlgoOrder(o) => Ok(AlgoMessage::AlgoOrder(o.algo_order)),
            TwapMessage::AlgoControl(c) => Ok(AlgoMessage::AlgoControl(*c)),
            TwapMessage::AlgoAck(a) => Ok(AlgoMessage::AlgoAck(*a)),
            TwapMessage::AlgoReject(r) => Ok(AlgoMessage::AlgoReject(*r)),
            TwapMessage::AlgoStatus(s) => Ok(AlgoMessage::AlgoStatus(s.algo_status.clone())),
            TwapMessage::TwapWakeup(_) => Err(()),
            TwapMessage::Orderflow(o) => Ok(AlgoMessage::Orderflow(*o)),
            TwapMessage::ChildAck(_) 
            | TwapMessage::ChildOut(_) 
            | TwapMessage::ChildFill(_) 
            | TwapMessage::ChildReject(_) 
            | TwapMessage::OrderIdAllocation(_) 
            | TwapMessage::BookUpdate(_) 
            | TwapMessage::GetTwapStatuses(_) 
            | TwapMessage::GetTwapStatusesResponse(..) => Err(())
        }
    }
}

impl TryInto<TwapMessage> for &AlgoMessage {
    type Error = ();

    fn try_into(self) -> Result<TwapMessage, ()> {
        match self {
            AlgoMessage::ChildReject(r) => Ok(TwapMessage::ChildReject(*r)),
            AlgoMessage::ChildAck(a) => Ok(TwapMessage::ChildAck(*a)),
            AlgoMessage::ChildFill(f) => Ok(TwapMessage::ChildFill(*f)),
            AlgoMessage::ChildOut(o) => Ok(TwapMessage::ChildOut(*o)),
            AlgoMessage::Orderflow(o) => Ok(TwapMessage::Orderflow(*o)),
            AlgoMessage::AlgoOrder(_) => Err(()),
            AlgoMessage::AlgoControl(c) => Ok(TwapMessage::AlgoControl(*c)),
            AlgoMessage::AlgoAck(a) => Ok(TwapMessage::AlgoAck(*a)),
            AlgoMessage::AlgoReject(r) => Ok(TwapMessage::AlgoReject(*r)),
            AlgoMessage::AlgoStatus(_) => Err(()),
            AlgoMessage::GetAlgoStatuses(_) => Err(()),
            AlgoMessage::GetAlgoStatusesResponse(..) => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct TwapOrder {
    #[serde(flatten)]
    pub algo_order: AlgoOrder,
    pub market: MarketId,
    pub dir: Dir,
    pub quantity: Decimal,
    pub interval: Duration,
    pub end_time: DateTime<Utc>,
    pub account: Option<Str>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct TwapStatus {
    #[serde(flatten)]
    pub twap_order: TwapOrder,
    #[serde(flatten)]
    pub algo_status: AlgoStatus,
    pub realized_twap: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct TwapWakeup {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, Copy, Pack, FromValue, Serialize, Deserialize)]
pub struct BookUpdate {
    pub market: MarketId,
    pub bbo: DirPair<Option<(Decimal, Decimal)>>,
}
