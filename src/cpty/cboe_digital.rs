use crate::{folio::FolioMessage, symbology::market::NormalizedMarketInfo};
use derive::FromStrJson;
#[cfg(feature = "netidx")]
use derive::FromValue;
#[cfg(feature = "netidx")]
use netidx_derive::Pack;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Debug, Clone, FromStrJson, Serialize, Deserialize, Zeroize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub struct CboeDigitalCreds {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
pub struct CboeDigitalMarketInfo {
    pub tick_size: Decimal,
    pub step_size: Decimal,
}

impl NormalizedMarketInfo for CboeDigitalMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.step_size
    }

    fn is_delisted(&self) -> bool {
        false
    }
}

impl std::fmt::Display for CboeDigitalMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "netidx", derive(Pack))]
#[cfg_attr(feature = "netidx", derive(FromValue))]
pub enum CboeDigitalMessage {
    Folio(FolioMessage),
}

impl TryInto<FolioMessage> for &CboeDigitalMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, Self::Error> {
        match self {
            CboeDigitalMessage::Folio(f) => Ok(f.clone()),
        }
    }
}

impl TryInto<CboeDigitalMessage> for &FolioMessage {
    type Error = ();

    fn try_into(self) -> Result<CboeDigitalMessage, Self::Error> {
        Ok(CboeDigitalMessage::Folio(self.clone()))
    }
}
