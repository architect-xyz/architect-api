use super::Ticker;
use crate::symbology::{PutOrCall, TradableProduct};
use chrono::NaiveDate;
use derive::grpc;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "options_expirations",
    response = "OptionsExpirations"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct OptionsExpirationsRequest {
    pub underlying: String,
    pub wrap: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct OptionsExpirations {
    pub underlying: String,
    pub wrap: String,
    pub expirations: Vec<NaiveDate>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "options_wraps", response = "OptionsWraps")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct OptionsWrapsRequest {
    pub underlying: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct OptionsWraps {
    pub underlying: String,
    pub wraps: Vec<String>,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "options_contract", response = "OptionsContract")]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct OptionsContractRequest {
    pub tradable_product: TradableProduct,
}

#[grpc(package = "json.architect")]
#[grpc(service = "Marketdata", name = "options_chain", response = "OptionsChain")]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct OptionsChainRequest {
    pub underlying: String,
    pub wrap: Option<String>,
    pub expiration: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsChain {
    pub calls: Vec<OptionsContract>,
    pub puts: Vec<OptionsContract>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsContract {
    pub ticker: Ticker,
    pub underlying: String,
    pub strike: Decimal,
    pub expiration: NaiveDate,
    pub put_or_call: PutOrCall,
    pub in_the_money: Option<bool>,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "options_contract_greeks",
    response = "OptionsGreeks"
)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct OptionsContractGreeksRequest {
    pub tradable_product: TradableProduct,
}

#[grpc(package = "json.architect")]
#[grpc(
    service = "Marketdata",
    name = "options_chain_greeks",
    response = "OptionsChainGreeks"
)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct OptionsChainGreeksRequest {
    pub underlying: String,
    pub wrap: Option<String>,
    pub expiration: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsChainGreeks {
    pub calls: Vec<OptionsGreeks>,
    pub puts: Vec<OptionsGreeks>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionsGreeks {
    pub symbol: String,
    pub underlying: String,
    pub strike: Decimal,
    pub expiration: NaiveDate,
    pub put_or_call: PutOrCall,
    pub delta: Decimal,
    pub gamma: Decimal,
    pub theta: Decimal,
    pub vega: Decimal,
    pub rho: Decimal,
    pub implied_volatility: Decimal,
}
