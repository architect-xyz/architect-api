//! Custom serializer/deserializer for chrono::DateTime<Utc>

use crate::json_schema_is_string;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_with::serde_conv;
use std::str::FromStr;

serde_conv!(
    pub DateTimeOrUtc,
    DateTime<Utc>,
    format_datetime_or_utc,
    parse_datetime_or_utc
);

fn format_datetime_or_utc(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_datetime_or_utc(s: &str) -> Result<DateTime<Utc>> {
    // First try parsing as RFC3339 (with timezone)
    if let Ok(dt) = DateTime::from_str(s) {
        return Ok(dt);
    }

    // If that fails, try parsing as NaiveDateTime (without timezone) and assume UTC
    match NaiveDateTime::from_str(s) {
        Ok(naive_dt) => Ok(Utc.from_utc_datetime(&naive_dt)),
        Err(e) => Err(anyhow!(
            "Failed to parse as DateTime<Utc> (with or without timezone): {} - {}",
            s,
            e
        )),
    }
}

json_schema_is_string!(DateTimeOrUtc, "date-time");
