use super::*;
use crate::{
    orderflow::{OrderIdAllocation, OrderflowMessage},
    symbology::{MarketId, ProductId},
    utils::messaging::MaybeRequest,
    Dir, HumanDuration, OrderId,
};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum SmartOrderRouterMessage {
    AlgoOrder(SmartOrderRouterOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    Orderflow(OrderflowMessage),
    ChildAck(ChildAck),
    ChildOut(ChildOut),
    ChildFill(ChildFill),
    ChildReject(ChildReject),
    OrderIdAllocation(OrderIdAllocation),
    PreviewAlgo(Uuid, SmartOrderRouterOrder),
    PreviewAlgoResponse(Uuid, Option<AlgoPreview>),
    GetAlgoStatus(Uuid, Option<OrderId>),
    GetAlgoStatusResponse(Uuid, Arc<Vec<AlgoStatus>>),
    GetAlgoLog(Uuid, OrderId),
    GetAlgoLogResponse(Uuid, AlgoLog),
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct SmartOrderRouterOrder {
    pub order_id: OrderId,
    pub markets: Arc<Vec<MarketId>>,
    pub base: ProductId,
    pub quote: ProductId,
    pub dir: Dir,
    pub limit_price: Decimal,
    pub target_size: Decimal,
    pub execution_time_limit: HumanDuration,
}

impl TryInto<AlgoOrder> for &SmartOrderRouterOrder {
    type Error = ();

    fn try_into(self) -> Result<AlgoOrder, ()> {
        Ok(AlgoOrder {
            order_id: self.order_id,
            algo: "SmartOrderRouter".try_into().map_err(|_| ())?,
        })
    }
}

impl TryInto<SmartOrderRouterMessage> for &AlgoMessage {
    type Error = ();

    fn try_into(self) -> Result<SmartOrderRouterMessage, ()> {
        use SmartOrderRouterMessage::*;
        match self {
            AlgoMessage::AlgoOrder(..) => Err(()),
            AlgoMessage::AlgoControl(c) => Ok(AlgoControl(*c)),
            AlgoMessage::AlgoAck(a) => Ok(AlgoAck(*a)),
            AlgoMessage::AlgoReject(r) => Ok(AlgoReject(r.clone())),
            AlgoMessage::AlgoStatus(s) => Ok(AlgoStatus(s.clone())),
            AlgoMessage::ChildAck(a) => Ok(ChildAck(*a)),
            AlgoMessage::ChildOut(o) => Ok(ChildOut(*o)),
            AlgoMessage::ChildFill(f) => Ok(ChildFill(*f)),
            AlgoMessage::ChildReject(r) => Ok(ChildReject(r.clone())),
            AlgoMessage::PreviewAlgo(..) => Err(()),
            AlgoMessage::PreviewAlgoResponse(id, p) => {
                Ok(PreviewAlgoResponse(*id, p.clone()))
            }
            AlgoMessage::GetAlgoOrder(..) => Err(()),
            AlgoMessage::GetAlgoOrderResponse(..) => Err(()),
            AlgoMessage::GetAlgoStatus(id, mo) => Ok(GetAlgoStatus(*id, *mo)),
            AlgoMessage::GetAlgoStatusResponse(id, ss) => {
                Ok(GetAlgoStatusResponse(*id, ss.clone()))
            }
            AlgoMessage::GetAlgoLog(id, o) => Ok(GetAlgoLog(*id, *o)),
            AlgoMessage::GetAlgoLogResponse(..) => Err(()),
        }
    }
}

impl TryInto<AlgoMessage> for &SmartOrderRouterMessage {
    type Error = ();

    fn try_into(self) -> Result<AlgoMessage, ()> {
        use SmartOrderRouterMessage::*;
        match self {
            AlgoOrder(o) => Ok(AlgoMessage::AlgoOrder(o.try_into()?)),
            AlgoControl(c) => Ok(AlgoMessage::AlgoControl(*c)),
            AlgoAck(a) => Ok(AlgoMessage::AlgoAck(*a)),
            AlgoReject(r) => Ok(AlgoMessage::AlgoReject(r.clone())),
            AlgoStatus(s) => Ok(AlgoMessage::AlgoStatus(s.clone())),
            Orderflow(..) => Err(()),
            ChildAck(a) => Ok(AlgoMessage::ChildAck(*a)),
            ChildOut(o) => Ok(AlgoMessage::ChildOut(*o)),
            ChildFill(f) => Ok(AlgoMessage::ChildFill(*f)),
            ChildReject(r) => Ok(AlgoMessage::ChildReject(r.clone())),
            OrderIdAllocation(..) => Err(()),
            GetAlgoStatus(_, Some(_)) => Err(()),
            PreviewAlgo(id, o) => Ok(AlgoMessage::PreviewAlgo(*id, o.try_into()?)),
            PreviewAlgoResponse(id, p) => {
                Ok(AlgoMessage::PreviewAlgoResponse(*id, p.clone()))
            }
            GetAlgoStatus(id, mo) => Ok(AlgoMessage::GetAlgoStatus(*id, *mo)),
            GetAlgoStatusResponse(id, ss) => {
                Ok(AlgoMessage::GetAlgoStatusResponse(*id, ss.clone()))
            }
            GetAlgoLog(id, o) => Ok(AlgoMessage::GetAlgoLog(*id, *o)),
            GetAlgoLogResponse(..) => Err(()),
        }
    }
}

impl MaybeRequest for SmartOrderRouterMessage {
    fn request_id(&self) -> Option<Uuid> {
        match self {
            Self::PreviewAlgo(id, _) | Self::GetAlgoStatus(id, _) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        match self {
            Self::PreviewAlgoResponse(id, _) | Self::GetAlgoStatusResponse(id, _) => {
                Some(*id)
            }
            _ => None,
        }
    }
}
