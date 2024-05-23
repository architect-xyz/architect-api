use super::*;
use derive::{FromInner, FromValue, TryIntoAnyInner};
use enumflags2::{bitflags, BitFlags};
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
#[transitive(B2C2Cpty <-> Orderflow)]
#[transitive(BinanceCpty <-> Orderflow)]
#[transitive(BinanceCpty <-> Folio)]
#[transitive(CboeDigitalCpty <-> Folio)]
#[transitive(CoinbaseCpty <-> Folio)]
#[transitive(CoinbaseCpty <-> Orderflow)]
#[transitive(CoinbasePrimeCpty <-> Folio)]
#[transitive(CoinbasePrimeCpty <-> Orderflow)]
#[transitive(CqgCpty <-> Orderflow)]
#[transitive(CqgCpty <-> Folio)]
#[transitive(CumberlandCpty <-> Orderflow)]
#[transitive(CumberlandCpty <-> Folio)]
#[transitive(DeribitCpty <-> Folio)]
#[transitive(DeribitCpty <-> Orderflow)]
#[transitive(FalconXCpty <-> Folio)]
#[transitive(FalconXCpty <-> Orderflow)]
#[transitive(GalaxyCpty <-> Orderflow)]
#[transitive(KrakenCpty <-> Folio)]
#[transitive(KrakenCpty <-> Orderflow)]
#[transitive(MockCpty <-> Folio)]
#[transitive(MockCpty <-> Orderflow)]
#[transitive(ExternalCpty <-> Folio)]
#[transitive(ExternalCpty <-> Orderflow)]
#[transitive(WintermuteCpty <-> Folio)]
#[transitive(WintermuteCpty <-> Orderflow)]
#[transitive(Orderflow <-> Oms)]
#[transitive(Orderflow <- Algo)]
#[transitive(Algo <-> TwapAlgo <- Orderflow)]
#[transitive(Algo <-> SmartOrderRouterAlgo)]
#[transitive(Algo <-> MMAlgo <- Orderflow)]
#[transitive(Algo <-> PovAlgo <- Orderflow)]
#[rustfmt::skip]
pub enum TypedMessage {
    #[pack(tag(  0))] SystemControl(system_control::SystemControlMessage),
    #[pack(tag(  1))] Symbology(symbology::SymbologyUpdate),
    #[pack(tag(  2))] OrderAuthority(orderflow::OrderAuthorityMessage),
    #[pack(tag(  3))] Orderflow(orderflow::OrderflowMessage),
    #[pack(tag(  4))] Oms(oms::OmsMessage),
    #[pack(tag(  5))] Algo(algo::AlgoMessage),
    #[pack(tag(  6))] Folio(folio::FolioMessage),
    #[pack(tag(  7))] AccountMaster(orderflow::account::AccountMessage),
    #[pack(tag( 98))] ExternalCpty(cpty::generic_external::ExternalCptyMessage),
    #[pack(tag( 99))] MockCpty(cpty::mock::MockCptyMessage),
    #[pack(tag(100))] CoinbaseCpty(cpty::coinbase::CoinbaseMessage),
    #[pack(tag(101))] B2C2Cpty(cpty::b2c2::B2C2Message),
    #[pack(tag(103))] KrakenCpty(cpty::kraken::KrakenMessage),
    #[pack(tag(104))] DeribitCpty(cpty::deribit::DeribitMessage),
    #[pack(tag(105))] WintermuteCpty(cpty::wintermute::WintermuteMessage),
    #[pack(tag(106))] FalconXCpty(cpty::falconx::FalconXMessage),
    #[pack(tag(107))] CoinbasePrimeCpty(cpty::coinbase_prime::CoinbasePrimeMessage),
    #[pack(tag(108))] GalaxyCpty(cpty::galaxy::GalaxyMessage),
    #[pack(tag(109))] CumberlandCpty(cpty::cumberland::CumberlandMessage),
    #[pack(tag(110))] CboeDigitalCpty(cpty::cboe_digital::CboeDigitalMessage),
    #[pack(tag(111))] BinanceCpty(cpty::binance::BinanceMessage),
    #[pack(tag(112))] CqgCpty(cpty::cqg::CqgMessage),
    #[pack(tag(200))] TwapAlgo(algo::twap::TwapMessage),
    #[pack(tag(201))] SmartOrderRouterAlgo(algo::smart_order_router::SmartOrderRouterMessage),
    #[pack(tag(202))] MMAlgo(algo::mm::MMAlgoMessage),
    #[pack(tag(203))] PovAlgo(algo::pov::PovAlgoMessage),
}

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
            TypedMessage::AccountMaster(am) => {
                use orderflow::account::AccountMessage;
                match am {
                    AccountMessage::MapAccount(..)
                    | AccountMessage::Accounts(None, _) => MessageTopic::Accounts.into(),
                    _ => BitFlags::empty(),
                }
            }
            _ => BitFlags::empty(),
        }
    }
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageTopic {
    Orderflow,
    ExternalOrderflow,
    Accounts,
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
        orderflow::{OrderBuilder, OrderId, OrderSource, Out},
        symbology::MarketId,
    };
    use anyhow::Result;
    use rust_decimal::Decimal;

    #[test]
    fn test_try_into_any_variant() -> Result<()> {
        use crate::orderflow::OrderflowMessage;
        let m = TypedMessage::Orderflow(OrderflowMessage::Order(
            OrderBuilder::new(
                OrderId::new_unchecked(123),
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

    /// test transitive closure of length 3 (B2C2 -> Orderflow -> Algo -> TWAPAlgo)
    #[test]
    fn test_try_into_any_variant_3() -> Result<()> {
        use crate::{algo::twap::TwapMessage, cpty::b2c2::B2C2Message};
        let src = TypedMessage::B2C2Cpty(B2C2Message::Out(Out {
            order_id: OrderId::new_unchecked(123),
        }));
        let dst: std::result::Result<MaybeSplit<TypedMessage, TwapMessage>, _> =
            src.try_into();
        assert_eq!(dst.is_ok(), true);
        Ok(())
    }
}
