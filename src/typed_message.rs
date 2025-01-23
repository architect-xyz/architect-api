#[cfg(feature = "netidx")]
use super::*;
#[cfg(feature = "netidx")]
use derive::{FromValue, TryIntoAnyInner};
#[cfg(feature = "netidx")]
use derive_more::From;
use enumflags2::bitflags;
#[cfg(feature = "netidx")]
use enumflags2::BitFlags;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
#[cfg(feature = "netidx")]
use schemars::JsonSchema;
use schemars::JsonSchema_repr;
#[cfg(feature = "netidx")]
use serde::{Deserialize, Serialize};
use serde_json::json;

/// TypedMessage is a wrapper enum for component messages, for all components that
/// this version of Architect is compiled with and supports.  This lets components
/// define and operate over their own independent message types while still allowing 
/// cross-component communication.
///
/// Architect installations are mutually intelligible to the extent of TypedMessage
/// variants they share in common.
///
/// TypedMessage should follow sensible rules for versioning and cross-
/// compatibility, such as explicit tagging of variants, and avoiding breaking 
/// changes to the component message types.
#[cfg(feature = "netidx")]
#[derive(Debug, Clone, Pack, From, FromValue, Serialize, Deserialize, TryIntoAnyInner, JsonSchema)]
#[transitive(CqgCpty <-> Orderflow)]
#[transitive(CqgCpty <-> Folio)]
#[transitive(PaperCpty <-> Folio)]
#[transitive(PaperCpty <-> Orderflow)]
#[transitive(Orderflow <-> Oms)]
#[transitive(Orderflow <- Algo)]
#[transitive(Algo <-> TwapAlgo <- Orderflow)]
#[transitive(Algo <-> ChaserAlgo <- Orderflow)]
#[transitive(Algo <-> SmartOrderRouterAlgo)]
#[transitive(Algo <-> MarketMakerAlgo <- Orderflow)]
#[transitive(Algo <-> PovAlgo <- Orderflow)]
#[transitive(TradingActivity <- Orderflow)]
#[transitive(Algo <-> SpreaderAlgo <- Orderflow)]
#[rustfmt::skip]
pub enum TypedMessage {
    #[pack(tag(  0))] SystemControl(system_control::SystemControlMessage),
    #[pack(tag(  1))] Symbology(symbology::SymbologyUpdate),
    #[pack(tag(  3))] Orderflow(orderflow::OrderflowMessage),
    #[pack(tag(  4))] Oms(oms::OmsMessage),
    #[pack(tag(  5))] Algo(algo::AlgoMessage),
    #[pack(tag(  6))] Folio(folio::FolioMessage),
    #[pack(tag( 10))] ChannelControl(channel_control::ChannelControlMessage),
    #[pack(tag(112))] CqgCpty(cpty::cqg::CqgMessage),
    #[pack(tag(115))] PaperCpty(cpty::paper::PaperCptyMessage),
    #[pack(tag(200))] TwapAlgo(algo::twap::TwapAlgoMessage),
    #[pack(tag(201))] SmartOrderRouterAlgo(algo::smart_order_router::SmartOrderRouterMessage),
    #[pack(tag(202))] MarketMakerAlgo(algo::mm::MMAlgoMessage),
    #[pack(tag(203))] PovAlgo(algo::pov::PovAlgoMessage),
    #[pack(tag(204))] ChaserAlgo(algo::chaser::ChaserAlgoMessage),
    #[pack(tag(205))] TradingActivity(trading_activity::TradingActivityMessage),
    #[pack(tag(206))] TakeAndChaseAlgo(algo::take_and_chase::TakeAndChaseAlgoMessage),
    #[pack(tag(207))] QuoteOneSideAlgo(algo::quote_one_side::QuoteOneSideAlgoMessage),
    #[pack(tag(208))] SpreaderAlgo(algo::spreader::SpreaderAlgoMessage)
}

#[cfg(feature = "netidx")]
impl TypedMessage {
    pub fn is_system_control(&self) -> bool {
        matches!(self, TypedMessage::SystemControl(..))
    }

    pub fn downcast<T>(self) -> Option<T>
    where
        TypedMessage: TryInto<MaybeSplit<TypedMessage, T>>,
    {
        if let Ok((_, downcasted)) =
            TryInto::<MaybeSplit<TypedMessage, T>>::try_into(self).map(MaybeSplit::parts)
        {
            Some(downcasted)
        } else {
            None
        }
    }

    pub fn topics(&self) -> BitFlags<MessageTopic> {
        match self {
            TypedMessage::Orderflow(_) => MessageTopic::Orderflow.into(),
            // CR alee: would be easier to determine in common+ext data format
            TypedMessage::Oms(om) => {
                use oms::OmsMessage;
                match om {
                    OmsMessage::Order(..)
                    | OmsMessage::OrderUpdate(..)
                    | OmsMessage::Cancel(..)
                    | OmsMessage::CancelAll(..)
                    | OmsMessage::Reject(..)
                    | OmsMessage::Ack(..)
                    | OmsMessage::Fill(..)
                    | OmsMessage::FillWarning(..)
                    | OmsMessage::Out(..) => MessageTopic::Orderflow.into(),
                    _ => BitFlags::empty(),
                }
            }
            _ => BitFlags::empty(),
        }
    }
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema_repr)]
pub enum MessageTopic {
    Orderflow,
    ExternalOrderflow,
}

pub enum MaybeSplit<A, B> {
    Just(B),
    Split(A, B),
}

impl<A, B> MaybeSplit<A, B> {
    pub fn parts(self) -> (Option<A>, B) {
        match self {
            MaybeSplit::Just(b) => (None, b),
            MaybeSplit::Split(a, b) => (Some(a), b),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        orderflow::{OrderBuilder, OrderId, OrderSource},
        symbology::MarketId,
        Dir,
    };
    use anyhow::Result;
    use rust_decimal::Decimal;

    #[test]
    fn test_try_into_any_variant() -> Result<()> {
        use crate::orderflow::OrderflowMessage;
        let m = TypedMessage::Orderflow(OrderflowMessage::Order(
            OrderBuilder::new(
                OrderId::nil(123),
                OrderSource::API,
                MarketId::try_from("BTC Crypto/USD*COINBASE/DIRECT")?,
            )
            .limit(Dir::Buy, Decimal::new(100, 0), Decimal::new(1, 0), false)
            .build()?,
        ));
        let m2: std::result::Result<MaybeSplit<TypedMessage, oms::OmsMessage>, _> =
            m.try_into();
        assert_eq!(m2.is_ok(), true);
        Ok(())
    }
}
