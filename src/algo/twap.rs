use super::{common_params::TakeThrough, *};
use crate::{
    symbology::{ExecutionVenue, MarketdataVenue},
    AccountIdOrName, Dir, HumanDuration,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Twap;

impl Algo for Twap {
    const NAME: &'static str = "TWAP";

    type Params = TwapParams;
    type Status = TwapStatus;
}

pub type TwapOrder = AlgoOrder<Twap>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "create_twap_order", response = "TwapOrder")]
pub type CreateTwapOrderRequest = CreateAlgoOrderRequest<Twap>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "modify_twap_order", response = "TwapOrder")]
pub type ModifyTwapOrderRequest = ModifyAlgoOrderRequest<Twap>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "twap_order", response = "TwapOrder")]
pub type TwapOrderRequest = AlgoOrderRequest;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "twap_orders", response = "TwapOrdersResponse")]
pub type TwapOrdersRequest = AlgoOrdersRequest;

pub type TwapOrdersResponse = AlgoOrdersResponse<Twap>;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwapParams {
    pub symbol: String,
    pub marketdata_venue: MarketdataVenue,
    pub execution_venue: ExecutionVenue,
    pub account: Option<AccountIdOrName>,
    pub dir: Dir,
    pub quantity: Decimal,
    pub interval: HumanDuration,
    /// The TWAP will finish within 1 interval of the end time.
    pub end_time: DateTime<Utc>,
    pub reject_lockout: HumanDuration,
    /// When placing an order, how aggressively to take.
    pub take_through: TakeThrough,
}

impl Validate for TwapParams {
    fn validate(&self) -> Result<()> {
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if self.interval.num_milliseconds() < 100 {
            bail!("interval must be >= 100ms");
        }
        // if self.reject_lockout.num_milliseconds() < 500
        //     || self.reject_lockout.num_seconds() > 300
        // {
        //     bail!("reject lockout must be between 0.5 seconds and 300 seconds");
        // }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwapStatus {
    pub realized_twap: Option<Decimal>,
    pub quantity_filled: Decimal,
}
