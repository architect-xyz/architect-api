use crate::{algo::*, utils::messaging::MaybeRequest, OrderId};
use bytes::Bytes;
use derive::FromValue;
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum AlgoContainerMessage<
    AlgoOrder: 'static,
    AlgoModify: 'static,
    AlgoPreview: 'static,
    AlgoStatus: 'static,
    AlgoLog: 'static,
> {
    AlgoOrder(AlgoOrder),
    AlgoControl(AlgoControl),
    AlgoAck(AlgoAck),
    AlgoReject(AlgoReject),
    AlgoStatus(AlgoStatus),
    AlgoOut(AlgoOut),
    ChildAck(ChildAck),
    ChildReject(ChildReject),
    ChildFill(ChildFill),
    ChildOut(ChildOut),
    Initialize,
    RetireStopped,
    Orderflow(OrderflowMessage),
    ChildOrderflow(OrderId, OrderflowMessage),
    ChildAlgoOrderflow(OrderId, ChildAlgoOrderflow),
    UpdateState(OrderId, Box<Bytes>),
    PreviewAlgo(Uuid, AlgoOrder),
    PreviewAlgoResponse(Uuid, Option<AlgoPreview>),
    GetAlgoOrder(Uuid, OrderId),
    GetAlgoOrderResponse(Uuid, Option<AlgoOrder>),
    GetAlgoStatus(Uuid, Option<OrderId>),
    GetAlgoStatusResponse(Uuid, Arc<Vec<AlgoStatus>>),
    GetAlgoLog(Uuid, OrderId),
    GetAlgoLogResponse(Uuid, Option<AlgoLog>),
    AlgoModify(Uuid, AlgoModify),
    AlgoModifyAccept(Uuid, AlgoModify),
    AlgoModifyReject(Uuid),
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum ChildAlgoOrderflow {
    // CR-someday arao: When redesigning, make this not explicitly enumerated
    ChildTwapOrder(twap::TwapOrder),
    ChildSmartOrderRouterOrder(smart_order_router::SmartOrderRouterOrder),
    ChildMarketMakerOrder(mm::MMAlgoOrder),
    ChildPovOrder(pov::PovAlgoOrder),
    ChildAlgoControl(AlgoControl),
    ChildChaserOrder(chaser::ChaserOrder),
}

macro_rules! time {
    ($x:expr) => {
        $x.try_into().map_err(|_| ())
    };
}

macro_rules! time_opt {
    ($x:expr) => {
        $x.as_ref().map(|x| time!(x)).transpose()
    };
}

impl<O, M, P, S, L> TryInto<AlgoMessage> for &AlgoContainerMessage<O, M, P, S, L>
where
    for<'a> &'a O: TryInto<AlgoOrder>,
    for<'a> &'a M: TryInto<AlgoModify>,
    for<'a> &'a P: TryInto<AlgoPreview>,
    for<'a> &'a S: TryInto<AlgoStatus>,
    for<'a> &'a L: TryInto<AlgoLog>,
{
    type Error = ();

    fn try_into(self) -> Result<AlgoMessage, ()> {
        use AlgoContainerMessage as ACM;
        use AlgoMessage as AM;
        match self {
            ACM::AlgoOrder(o) => Ok(AM::AlgoOrder(time!(o)?)),
            ACM::AlgoControl(c) => Ok(AM::AlgoControl(*c)),
            ACM::AlgoAck(a) => Ok(AM::AlgoAck(*a)),
            ACM::AlgoOut(o) => Ok(AM::AlgoOut(*o)),
            ACM::AlgoReject(r) => Ok(AM::AlgoReject(r.clone())),
            ACM::AlgoStatus(s) => Ok(AM::AlgoStatus(time!(s)?)),
            ACM::AlgoModify(id, a) => Ok(AM::AlgoModify(*id, time!(a)?)),
            ACM::AlgoModifyAccept(id, a) => Ok(AM::AlgoModifyAccept(*id, time!(a)?)),
            ACM::AlgoModifyReject(id) => Ok(AM::AlgoModifyReject(*id)),
            ACM::ChildAck(a) => Ok(AM::ChildAck(*a)),
            ACM::ChildReject(r) => Ok(AM::ChildReject(r.clone())),
            ACM::ChildFill(f) => Ok(AM::ChildFill(*f)),
            ACM::ChildOut(o) => Ok(AM::ChildOut(*o)),
            ACM::Initialize
            | ACM::RetireStopped
            | ACM::Orderflow(..)
            | ACM::ChildOrderflow(..)
            | ACM::ChildAlgoOrderflow(..)
            | ACM::UpdateState(..) => Err(()),
            ACM::PreviewAlgo(id, o) => Ok(AM::PreviewAlgo(*id, time!(o)?)),
            ACM::PreviewAlgoResponse(id, p) => {
                Ok(AM::PreviewAlgoResponse(*id, time_opt!(p)?))
            }
            ACM::GetAlgoOrder(id, oid) => Ok(AM::GetAlgoOrder(*id, *oid)),
            ACM::GetAlgoOrderResponse(id, o) => {
                Ok(AM::GetAlgoOrderResponse(*id, time_opt!(o)?))
            }
            ACM::GetAlgoStatus(id, oid) => Ok(AM::GetAlgoStatus(*id, *oid)),
            ACM::GetAlgoStatusResponse(id, s) => {
                // CR-someday alee: a bit sad in the case of S = AlgoStatus re allocations
                let s: Vec<AlgoStatus> =
                    s.iter().map(|s| time!(s)).collect::<Result<_, _>>()?;
                Ok(AM::GetAlgoStatusResponse(*id, Arc::new(s)))
            }
            ACM::GetAlgoLog(id, oid) => Ok(AM::GetAlgoLog(*id, *oid)),
            ACM::GetAlgoLogResponse(id, l) => {
                Ok(AM::GetAlgoLogResponse(*id, time_opt!(l)?))
            }
        }
    }
}

impl<O, M, P, S, L> TryInto<AlgoContainerMessage<O, M, P, S, L>> for &AlgoMessage {
    type Error = ();

    fn try_into(self) -> Result<AlgoContainerMessage<O, M, P, S, L>, ()> {
        use AlgoContainerMessage as ACM;
        use AlgoMessage as AM;
        match self {
            AM::AlgoOrder(..)
            | AM::AlgoModify(_, _)
            | AM::AlgoModifyAccept(_, _)
            | AM::AlgoModifyReject(_) => Err(()),
            AM::AlgoControl(c) => Ok(ACM::AlgoControl(*c)),
            AM::AlgoAck(a) => Ok(ACM::AlgoAck(*a)),
            AM::AlgoOut(o) => Ok(ACM::AlgoOut(*o)),
            AM::AlgoReject(r) => Ok(ACM::AlgoReject(r.clone())),
            AM::AlgoStatus(..) => Err(()),
            AM::ChildAck(a) => Ok(ACM::ChildAck(*a)),
            AM::ChildReject(r) => Ok(ACM::ChildReject(r.clone())),
            AM::ChildFill(f) => Ok(ACM::ChildFill(*f)),
            AM::ChildOut(o) => Ok(ACM::ChildOut(*o)),
            AM::PreviewAlgo(..) | AM::PreviewAlgoResponse(..) => Err(()),
            AM::GetAlgoOrder(id, oid) => Ok(ACM::GetAlgoOrder(*id, *oid)),
            AM::GetAlgoOrderResponse(..) => Err(()),
            AM::GetAlgoStatus(id, oid) => Ok(ACM::GetAlgoStatus(*id, *oid)),
            AM::GetAlgoStatusResponse(..) => Err(()),
            AM::GetAlgoLog(id, oid) => Ok(ACM::GetAlgoLog(*id, *oid)),
            AM::GetAlgoLogResponse(..) => Err(()),
        }
    }
}

impl<O, M, P, S, L> Into<AlgoContainerMessage<O, M, P, S, L>> for &OrderflowMessage {
    fn into(self) -> AlgoContainerMessage<O, M, P, S, L> {
        AlgoContainerMessage::Orderflow(self.clone())
    }
}

impl<O, M, P, S, L> MaybeRequest for AlgoContainerMessage<O, M, P, S, L> {
    fn request_id(&self) -> Option<Uuid> {
        use AlgoContainerMessage as ACM;
        match self {
            ACM::PreviewAlgo(id, _)
            | ACM::GetAlgoStatus(id, _)
            | ACM::GetAlgoLog(id, _) => Some(*id),
            _ => None,
        }
    }

    fn response_id(&self) -> Option<Uuid> {
        use AlgoContainerMessage as ACM;
        match self {
            ACM::PreviewAlgoResponse(id, _)
            | ACM::GetAlgoStatusResponse(id, _)
            | ACM::GetAlgoLogResponse(id, _) => Some(*id),
            _ => None,
        }
    }
}
