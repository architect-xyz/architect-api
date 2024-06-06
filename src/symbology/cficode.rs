//! Classification of Finacial Instruments
//! https://en.wikipedia.org/wiki/ISO_10962

#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

pub mod equity {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Voting {
        Voting,
        NonVoting,
        Restricted,
        EnhancedVoting,
        NA,
    }

    impl From<&[u8; 8]> for Voting {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'V' => Self::Voting,
                b'N' => Self::NonVoting,
                b'R' => Self::Restricted,
                b'E' => Self::EnhancedVoting,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Ownership {
        Restrictions,
        Free,
        NA,
    }

    impl From<&[u8; 8]> for Ownership {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'T' => Self::Restrictions,
                b'U' => Self::Free,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Payment {
        FullyPaid,
        NilPaid,
        PartlyPaid,
        NA,
    }

    impl From<&[u8; 8]> for Payment {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'F' => Self::FullyPaid,
                b'O' => Self::NilPaid,
                b'P' => Self::PartlyPaid,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Form {
        Bearer,
        Registered,
        BearerOrRegistered,
        BearerDepositoryReceipt,
        RegisteredDepositoryReceipt,
        Others,
        NA,
    }

    impl From<&[u8; 8]> for Form {
        fn from(value: &[u8; 8]) -> Self {
            match value[5] {
                b'B' => Self::Bearer,
                b'R' => Self::Registered,
                b'N' => Self::BearerOrRegistered,
                b'M' => Self::Others,
                b'Z' => Self::BearerDepositoryReceipt,
                b'A' => Self::RegisteredDepositoryReceipt,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Redemption {
        Redeemable,
        Extendible,
        Exchangeable,
        RedeemableOrExtendible,
        RedeemableOrExchangable,
        RedeemableOrExchangableOrExtendible,
        Perpetual,
        NA,
    }

    impl From<&[u8; 8]> for Redemption {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'R' => Self::Redeemable,
                b'E' => Self::Extendible,
                b'T' => Self::RedeemableOrExtendible,
                b'G' => Self::Exchangeable,
                b'A' => Self::RedeemableOrExchangableOrExtendible,
                b'C' => Self::RedeemableOrExchangable,
                b'N' => Self::Perpetual,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Income {
        FixedRate,
        CumulativeFixedRate,
        Participating,
        CumulativeParticipating,
        AdjustableOrVariableRate,
        NormalRate,
        AuctionRate,
        Dividends,
        NA,
    }

    impl From<&[u8; 8]> for Income {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'F' => Self::FixedRate,
                b'C' => Self::CumulativeFixedRate,
                b'P' => Self::Participating,
                b'Q' => Self::CumulativeParticipating,
                b'A' => Self::AdjustableOrVariableRate,
                b'N' => Self::NormalRate,
                b'U' => Self::AuctionRate,
                b'D' => Self::Dividends,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum DepositoryRedemption {
        Redeemable,
        Perpetual,
        Convertible,
        ConvertibleRedeemable,
        NA,
    }

    impl From<&[u8; 8]> for DepositoryRedemption {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'R' => Self::Redeemable,
                b'N' => Self::Perpetual,
                b'B' => Self::Convertible,
                b'D' => Self::ConvertibleRedeemable,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Dependency {
        Common,
        Preferred,
        CommonConvertible,
        PreferredConvertible,
        LimitedPartnershipUnits,
        Misc,
        NA,
    }

    impl From<&[u8; 8]> for Dependency {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'S' => Self::Common,
                b'P' => Self::Preferred,
                b'C' => Self::CommonConvertible,
                b'F' => Self::PreferredConvertible,
                b'L' => Self::LimitedPartnershipUnits,
                b'M' => Self::Misc,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommonAttrs {
        pub voting: Voting,
        pub ownership: Ownership,
        pub payment: Payment,
        pub form: Form,
    }

    impl From<&[u8; 8]> for CommonAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                voting: value.into(),
                ownership: value.into(),
                payment: value.into(),
                form: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PreferredAttrs {
        pub voting: Voting,
        pub redemption: Redemption,
        pub income: Income,
        pub form: Form,
    }

    impl From<&[u8; 8]> for PreferredAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                voting: value.into(),
                redemption: value.into(),
                income: value.into(),
                form: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DepositoryReceiptAttrs {
        pub instrument_dependency: Dependency,
        pub redemption: DepositoryRedemption,
        pub income: Income,
        pub form: Form,
    }

    impl From<&[u8; 8]> for DepositoryReceiptAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                instrument_dependency: value.into(),
                redemption: value.into(),
                income: value.into(),
                form: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum StructuredType {
        TrackerCertificate,
        OutperformingCertificate,
        BonusCertificate,
        OutperformanceBonusCertificate,
        TwinWinCertificate,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for StructuredType {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'A' => Self::TrackerCertificate,
                b'B' => Self::OutperformingCertificate,
                b'C' => Self::BonusCertificate,
                b'D' => Self::OutperformanceBonusCertificate,
                b'E' => Self::TwinWinCertificate,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum StructuredPayment {
        DividendPayments,
        NoPayments,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for StructuredPayment {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'D' => Self::DividendPayments,
                b'Y' => Self::NoPayments,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum StructuredRepayment {
        Cash,
        Physical,
        ElectAtSettlement,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for StructuredRepayment {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'F' => Self::Cash,
                b'V' => Self::Physical,
                b'E' => Self::ElectAtSettlement,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum StructuredUnderlying {
        Baskets,
        Equities,
        DebtInstruments,
        Derivatives,
        Commodities,
        Currencies,
        Indices,
        InterestRates,
        Others,
        NA,
    }

    impl From<&[u8; 8]> for StructuredUnderlying {
        fn from(value: &[u8; 8]) -> Self {
            match value[5] {
                b'B' => Self::Baskets,
                b'S' => Self::Equities,
                b'D' => Self::DebtInstruments,
                b'G' => Self::Derivatives,
                b'T' => Self::Commodities,
                b'C' => Self::Currencies,
                b'I' => Self::Indices,
                b'N' => Self::InterestRates,
                b'M' => Self::Others,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct StructuredAttrs {
        pub structured_type: StructuredType,
        pub payment: StructuredPayment,
        pub repayment: StructuredRepayment,
        pub underlying: StructuredUnderlying,
    }

    impl From<&[u8; 8]> for StructuredAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                structured_type: value.into(),
                payment: value.into(),
                repayment: value.into(),
                underlying: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ClosedOpen {
        Closed,
        Open,
        NA,
    }

    impl From<&[u8; 8]> for ClosedOpen {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'C' => Self::Closed,
                b'O' => Self::Open,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum UnitsDistPolicy {
        IncomeFunds,
        GrowthFunds,
        MixedFunds,
        NA,
    }

    impl From<&[u8; 8]> for UnitsDistPolicy {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'I' => Self::IncomeFunds,
                b'G' => Self::GrowthFunds,
                b'M' => Self::MixedFunds,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum UnitsAssets {
        RealEstate,
        Securities,
        MixedGeneral,
        Commodities,
        Derivitives,
        NA,
    }

    impl From<&[u8; 8]> for UnitsAssets {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'R' => Self::RealEstate,
                b'S' => Self::Securities,
                b'M' => Self::MixedGeneral,
                b'C' => Self::Commodities,
                b'D' => Self::Derivitives,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct UnitsAttrs {
        pub closed_open: ClosedOpen,
        pub distribution_policy: UnitsDistPolicy,
        pub assets: UnitsAssets,
        pub form: Form,
    }

    impl From<&[u8; 8]> for UnitsAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                closed_open: value.into(),
                distribution_policy: value.into(),
                assets: value.into(),
                form: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Common(CommonAttrs),
        Preferred(PreferredAttrs),
        Convertible(CommonAttrs),
        PreferredConvertible(PreferredAttrs),
        LimitedPartnershipUnits(CommonAttrs),
        DepositoryReceipts(DepositoryReceiptAttrs),
        StructuredInstruments(StructuredAttrs),
        PreferenceShares(PreferredAttrs),
        PreferenceConvertibleShares(PreferredAttrs),
        Units(UnitsAttrs),
        Misc(Form),
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'S' => Self::Common(value.into()),
                b'P' => Self::Preferred(value.into()),
                b'C' => Self::Convertible(value.into()),
                b'F' => Self::PreferredConvertible(value.into()),
                b'L' => Self::LimitedPartnershipUnits(value.into()),
                b'D' => Self::DepositoryReceipts(value.into()),
                b'Y' => Self::StructuredInstruments(value.into()),
                b'R' => Self::PreferenceShares(value.into()),
                b'V' => Self::PreferenceConvertibleShares(value.into()),
                b'U' => Self::Units(value.into()),
                b'M' => Self::Misc(value.into()),
                _ => Self::NA,
            }
        }
    }
}

pub mod options {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Exercise {
        American,
        European,
        Bermudan,
        NA,
    }

    impl From<&[u8; 8]> for Exercise {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'A' => Self::American,
                b'E' => Self::European,
                b'B' => Self::Bermudan,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Underlying {
        Baskets,
        Equities,
        Debt,
        Commodities,
        Currencies,
        Indices,
        Options,
        Futures,
        Swaps,
        InterestRates,
        Others,
        NA,
    }

    impl From<&[u8; 8]> for Underlying {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'B' => Self::Baskets,
                b'S' => Self::Equities,
                b'D' => Self::Debt,
                b'T' => Self::Commodities,
                b'C' => Self::Currencies,
                b'I' => Self::Indices,
                b'O' => Self::Options,
                b'F' => Self::Futures,
                b'W' => Self::Swaps,
                b'N' => Self::InterestRates,
                b'M' => Self::Others,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Delivery {
        Physical,
        Cash,
        NonDeliverable,
        ElectAtExercise,
        NA,
    }

    impl From<&[u8; 8]> for Delivery {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'P' => Self::Physical,
                b'C' => Self::Cash,
                b'N' => Self::NonDeliverable,
                b'E' => Self::ElectAtExercise,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Standard {
        Standard,
        NonStandard,
        NA,
    }

    impl From<&[u8; 8]> for Standard {
        fn from(value: &[u8; 8]) -> Self {
            match value[5] {
                b'S' => Self::Standard,
                b'N' => Self::NonStandard,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Attrs {
        pub exercise: Exercise,
        pub underlying: Underlying,
        pub delivery: Delivery,
        pub standard: Standard,
    }

    impl From<&[u8; 8]> for Attrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                exercise: value.into(),
                underlying: value.into(),
                delivery: value.into(),
                standard: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Put(Attrs),
        Call(Attrs),
        Other,
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'P' => Self::Put(value.into()),
                b'C' => Self::Call(value.into()),
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }
}

pub mod futures {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FinancialUnderlying {
        Baskets,
        Equities,
        Debt,
        Currencies,
        Indices,
        Options,
        Futures,
        Swaps,
        InterestRates,
        StockDividends,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for FinancialUnderlying {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'B' => Self::Baskets,
                b'S' => Self::Equities,
                b'D' => Self::Debt,
                b'C' => Self::Currencies,
                b'I' => Self::Indices,
                b'O' => Self::Options,
                b'F' => Self::Futures,
                b'W' => Self::Swaps,
                b'N' => Self::InterestRates,
                b'V' => Self::StockDividends,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CommodityUnderlying {
        ExtractionResources,
        Agriculture,
        IndustrialProducts,
        Services,
        Environmental,
        PolypropyleneProducts,
        GeneratedResources,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for CommodityUnderlying {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'E' => Self::ExtractionResources,
                b'A' => Self::Agriculture,
                b'I' => Self::IndustrialProducts,
                b'S' => Self::Services,
                b'N' => Self::Environmental,
                b'P' => Self::PolypropyleneProducts,
                b'H' => Self::GeneratedResources,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Delivery {
        Physical,
        Cash,
        NonDeliverable,
        NA,
    }

    impl From<&[u8; 8]> for Delivery {
        fn from(value: &[u8; 8]) -> Self {
            match value[3] {
                b'P' => Self::Physical,
                b'C' => Self::Cash,
                b'N' => Self::NonDeliverable,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Standard {
        Standardized,
        NonStandardized,
        NA,
    }

    impl From<&[u8; 8]> for Standard {
        fn from(value: &[u8; 8]) -> Self {
            match value[4] {
                b'S' => Self::Standardized,
                b'N' => Self::NonStandardized,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FinancialAttrs {
        pub underlying: FinancialUnderlying,
        pub delivery: Delivery,
        pub standard: Standard,
    }

    impl From<&[u8; 8]> for FinancialAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                underlying: value.into(),
                delivery: value.into(),
                standard: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommodityAttrs {
        pub underlying: CommodityUnderlying,
        pub delivery: Delivery,
        pub standard: Standard,
    }

    impl From<&[u8; 8]> for CommodityAttrs {
        fn from(value: &[u8; 8]) -> Self {
            Self {
                underlying: value.into(),
                delivery: value.into(),
                standard: value.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Financial(FinancialAttrs),
        Commodities(CommodityAttrs),
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'F' => Self::Financial(value.into()),
                b'C' => Self::Commodities(value.into()),
                _ => Self::NA,
            }
        }
    }
}

pub mod swaps {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Rates,
        Commodities,
        Equity,
        Credit,
        Forex,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'R' => Self::Rates,
                b'T' => Self::Commodities,
                b'E' => Self::Equity,
                b'C' => Self::Credit,
                b'F' => Self::Forex,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }
}

pub mod spot {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Kind {
        Agriculture,
        Energy,
        Metals,
        Environmental,
        PolypropyleneProducts,
        Fertilizer,
        Paper,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for Kind {
        fn from(value: &[u8; 8]) -> Self {
            match value[2] {
                b'A' => Self::Agriculture,
                b'J' => Self::Energy,
                b'K' => Self::Metals,
                b'N' => Self::Environmental,
                b'P' => Self::PolypropyleneProducts,
                b'S' => Self::Fertilizer,
                b'T' => Self::Paper,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Forex,
        Commodities(Kind),
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'F' => Self::Forex,
                b'T' => Self::Commodities(value.into()),
                _ => Self::NA,
            }
        }
    }
}

pub mod forwards {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Equity,
        Forex,
        Credit,
        Rates,
        Commodities,
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'E' => Self::Equity,
                b'F' => Self::Forex,
                b'C' => Self::Credit,
                b'R' => Self::Rates,
                b'T' => Self::Commodities,
                _ => Self::NA,
            }
        }
    }
}

pub mod strategies {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Class {
        Equity,
        Forex,
        Credit,
        Rates,
        Commodities,
        MixedAssets,
        Other,
        NA,
    }

    impl From<&[u8; 8]> for Class {
        fn from(value: &[u8; 8]) -> Self {
            match value[1] {
                b'R' => Self::Rates,
                b'T' => Self::Commodities,
                b'E' => Self::Equity,
                b'C' => Self::Credit,
                b'F' => Self::Forex,
                b'Y' => Self::MixedAssets,
                b'M' => Self::Other,
                _ => Self::NA,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    Equity(equity::Class),
    Debt,
    CollectiveInvestmentVehicles,
    Rights,
    Options(options::Class),
    Futures(futures::Class),
    Swaps(swaps::Class),
    UnlistedAndComplexOptions,
    Spot(spot::Class),
    Forwards(forwards::Class),
    Strategies(strategies::Class),
    Financing,
    ReferentialInstruments,
    Misc,
    NA,
}

impl From<&CfiCode> for Class {
    fn from(value: &CfiCode) -> Self {
        match value.0[0] {
            b'E' => Self::Equity((&value.0).into()),
            b'D' => Self::Debt,
            b'C' => Self::CollectiveInvestmentVehicles,
            b'R' => Self::Rights,
            b'O' => Self::Options((&value.0).into()),
            b'F' => Self::Futures((&value.0).into()),
            b'S' => Self::Swaps((&value.0).into()),
            b'H' => Self::UnlistedAndComplexOptions,
            b'I' => Self::Spot((&value.0).into()),
            b'J' => Self::Forwards((&value.0).into()),
            b'K' => Self::Strategies((&value.0).into()),
            b'L' => Self::Financing,
            b'T' => Self::ReferentialInstruments,
            b'M' => Self::Misc,
            _ => Self::NA,
        }
    }
}

impl From<CfiCode> for Class {
    fn from(value: CfiCode) -> Self {
        (&value).into()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[cfg_attr(feature = "netidx", derive(Pack, FromValue))]
pub struct CfiCode(pub [u8; 8]);

impl Default for CfiCode {
    fn default() -> Self {
        let x = b'X';
        CfiCode([x, x, x, x, x, x, x, x])
    }
}

impl From<&str> for CfiCode {
    fn from(value: &str) -> Self {
        let mut t = Self::default();
        for (i, c) in value.chars().enumerate() {
            if i > 5 {
                break;
            }
            if c.is_ascii() {
                t.0[i] = c as u8;
            }
        }
        t
    }
}

impl From<&[u8]> for CfiCode {
    fn from(value: &[u8]) -> Self {
        let mut t = Self::default();
        for (i, c) in value.iter().enumerate() {
            t.0[i] = *c;
        }
        t
    }
}

impl From<u64> for CfiCode {
    fn from(value: u64) -> Self {
        Self(value.to_be_bytes())
    }
}

impl Into<u64> for CfiCode {
    fn into(self) -> u64 {
        u64::from_be_bytes(self.0)
    }
}

impl CfiCode {
    pub fn classify(&self) -> Class {
        self.into()
    }
}
