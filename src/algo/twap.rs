use super::*;
use crate::{symbology::ExecutionVenue, Dir, HumanDuration};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use derive::grpc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwapAlgo;

impl Algo for TwapAlgo {
    type Params = TwapParams;
    type Status = TwapStatus;
}

pub type TwapAlgoOrder = AlgoOrder<TwapAlgo>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "create_twap_algo_order", response = "TwapAlgoOrder")]
pub type CreateTwapAlgoOrderRequest = CreateAlgoOrderRequest<TwapAlgo>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "modify_twap_algo_order", response = "TwapAlgoOrder")]
pub type ModifyTwapAlgoOrderRequest = ModifyAlgoOrderRequest<TwapAlgo>;

#[grpc(package = "json.architect")]
#[grpc(service = "Algo", name = "twap_algo_order", response = "TwapAlgoOrder")]
pub type TwapAlgoOrderRequest = AlgoOrderRequest;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwapParams {
    pub symbol: String,
    pub execution_venue: ExecutionVenue,
    pub dir: Dir,
    pub quantity: Decimal,
    pub interval: HumanDuration,
    pub end_time: DateTime<Utc>,
    pub reject_lockout: HumanDuration,
    pub take_through_frac: Option<Decimal>,
}

impl Validate for TwapParams {
    fn validate(&self) -> Result<()> {
        if !self.quantity.is_sign_positive() {
            bail!("quantity must be positive");
        }
        if self.interval.num_milliseconds() < 100 {
            bail!("interval must be >= 100ms");
        }
        if self.reject_lockout.num_milliseconds() < 500
            || self.reject_lockout.num_seconds() > 300
        {
            bail!("reject lockout must be between 0.5 seconds and 300 seconds");
        }
        if let Some(take_through_frac) = self.take_through_frac {
            if take_through_frac.is_sign_negative() || take_through_frac > dec!(0.05) {
                bail!("take_through_frac must be between 0 and 5%");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwapStatus {
    pub realized_twap: Option<Decimal>,
    pub quantity_filled: Decimal,
}
