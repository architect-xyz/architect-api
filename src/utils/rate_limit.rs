use crate::NonZeroDurationAsStr;
use anyhow::Result;
use governor::Quota;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, serde_conv};
use std::{num::NonZeroU32, time::Duration};

#[serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct RateLimit {
    pub max: NonZeroU32,
    #[serde_as(as = "NonZeroDurationAsStr")]
    #[schemars(with = "NonZeroDurationAsStr")]
    pub per: Duration,
}

impl RateLimit {
    pub fn as_quota(&self) -> governor::Quota {
        governor::Quota::with_period(self.per)
            .unwrap() // NonZeroDurationAsStr ensures this is non-zero
            .allow_burst(self.max)
    }
}

impl From<&Quota> for RateLimit {
    fn from(quota: &Quota) -> Self {
        RateLimit { max: quota.burst_size(), per: quota.replenish_interval() }
    }
}

serde_conv!(
    pub QuotaAsRateLimit,
    Quota,
    RateLimit::from,
    try_into_quota
);

fn try_into_quota(rate_limit: RateLimit) -> Result<Quota, std::convert::Infallible> {
    Ok(rate_limit.as_quota())
}
