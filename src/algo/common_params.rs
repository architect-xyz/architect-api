use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, JsonSchema)]
pub enum TakeThrough {
    Fraction(Decimal), // same as percent but 100x
    Percent(Decimal),
    Price(Decimal),
    Ticks(Decimal),
}

impl std::fmt::Display for TakeThrough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TakeThrough::Fraction(frac) => write!(f, "{frac}f"),
            TakeThrough::Percent(pct) => write!(f, "{pct}%"),
            TakeThrough::Price(price) => write!(f, "{price}"),
            TakeThrough::Ticks(ticks) => write!(f, "{ticks}t"),
        }
    }
}

impl std::str::FromStr for TakeThrough {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('f') {
            let num_str = s.strip_suffix('f').unwrap();
            let num = num_str.parse::<Decimal>()?;
            Ok(TakeThrough::Fraction(num))
        } else if s.ends_with('%') {
            let num_str = s.strip_suffix('%').unwrap();
            let num = num_str.parse::<Decimal>()?;
            Ok(TakeThrough::Percent(num))
        } else if s.ends_with('t') {
            let num_str = s.strip_suffix('t').unwrap();
            let num = num_str.parse::<Decimal>()?;
            Ok(TakeThrough::Ticks(num))
        } else {
            let num = s.parse::<Decimal>()?;
            Ok(TakeThrough::Price(num))
        }
    }
}
