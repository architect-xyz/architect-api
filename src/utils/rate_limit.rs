use crate::{utils::duration::parse_duration, NonZeroDurationAsStr};
use anyhow::{anyhow, Result};
use governor::Quota;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, serde_conv};
use std::{num::NonZeroU32, str::FromStr, time::Duration};

#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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

impl FromStr for RateLimit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (max, per) =
            s.split_once('/').ok_or_else(|| anyhow!("invalid rate limit"))?;
        Ok(RateLimit {
            max: max.trim().parse()?,
            per: parse_duration(per.trim())?.to_std()?,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_rate_limit() {
        let rate_limit = RateLimit {
            max: NonZeroU32::new(100).unwrap(),
            per: Duration::from_secs(60 * 5),
        };
        insta::assert_json_snapshot!(rate_limit, @r#"
        {
          "max": 100,
          "per": "300.000000000s"
        }
        "#);
        let parsed_rate_limit = r#"
        {
          "max": 100,
          "per": "300.000000000s"
        }
        "#;
        let parsed_rate_limit: RateLimit =
            serde_json::from_str(parsed_rate_limit).unwrap();
        assert_eq!(rate_limit, parsed_rate_limit);
        // test roundtrip of serde_json::Value
        let json_value = serde_json::to_value(rate_limit).unwrap();
        let parsed: RateLimit = serde_json::from_value(json_value).unwrap();
        assert_eq!(rate_limit, parsed);
    }
}
