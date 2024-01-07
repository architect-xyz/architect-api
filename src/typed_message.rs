use super::*;
use derive::{FromInner, FromValue, TryIntoAnyInner};
use netidx_derive::Pack;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, FromInner, TryIntoAnyInner)]
#[transitive(CoinbaseCpty -> Orderflow)]
#[transitive(B2C2Cpty <-> Orderflow)]
#[transitive(Orderflow -> Algo)]
#[transitive(Orderflow <-> Oms)]
#[transitive(Algo <-> TwapAlgo)]
#[rustfmt::skip]
pub enum TypedMessage {
    #[pack(tag(  0))] SystemControl(system_control::SystemControlMessage),
    #[pack(tag(  1))] Symbology(symbology::SymbologyUpdate),
    #[pack(tag(  2))] ChannelAuthority(orderflow::ChannelAuthorityMessage),
    #[pack(tag(  3))] Orderflow(orderflow::OrderflowMessage),
    #[pack(tag(  4))] Oms(oms::OmsMessage),
    #[pack(tag(  5))] Algo(orderflow::algo::AlgoMessage),
    #[pack(tag(100))] CoinbaseCpty(cpty::coinbase::CoinbaseMessage),
    #[pack(tag(101))] B2C2Cpty(cpty::b2c2::B2C2Message),
    #[pack(tag(103))] KrakenCpty(cpty::kraken::KrakenMessage),
    #[pack(tag(200))] TwapAlgo(algo::twap::TwapMessage),
}

impl TypedMessage {
    pub fn is_system_control(&self) -> bool {
        matches!(self, TypedMessage::SystemControl(..))
    }
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
        orderflow::{ChannelId, OrderIdGenerator, Out},
        symbology::MarketId,
    };
    use anyhow::Result;
    use rust_decimal::Decimal;

    #[test]
    fn test_try_into_any_variant() -> Result<()> {
        use crate::orderflow::{Order, OrderflowMessage};
        let oids = OrderIdGenerator::channel(ChannelId::new(0x10000)?)?;
        let m = TypedMessage::Orderflow(OrderflowMessage::Order(
            Order::builder(
                oids.next(),
                MarketId::try_from("BTC Crypto/USD*COINBASE/DIRECT")?,
                Dir::Buy,
                Decimal::new(100, 0),
                Decimal::new(1, 0),
            )
            .build()?,
        ));
        let m2: std::result::Result<MaybeSplit<TypedMessage, oms::OmsMessage>, _> =
            m.try_into();
        assert_eq!(m2.is_ok(), true);
        Ok(())
    }

    /// test transitive closure of length 3 (B2C2 -> Orderflow -> Algo -> TWAPAlgo)
    #[test]
    fn test_try_into_any_variant_3() -> Result<()> {
        use crate::{algo::twap::TwapMessage, cpty::b2c2::B2C2Message};
        let oids = OrderIdGenerator::channel(ChannelId::new(0x10000)?)?;
        let src = TypedMessage::B2C2Cpty(B2C2Message::Out(Out { order_id: oids.next() }));
        let dst: std::result::Result<MaybeSplit<TypedMessage, TwapMessage>, _> =
            src.try_into();
        assert_eq!(dst.is_ok(), true);
        Ok(())
    }
}
