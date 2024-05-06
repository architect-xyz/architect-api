use crate::{folio::FolioMessage, symbology::market::NormalizedMarketInfo};
use derive::{FromStrJson, FromValue};
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Debug, Clone, Pack, FromValue, FromStrJson, Serialize, Deserialize, Zeroize)]
pub struct CboeDigitalCreds {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
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

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
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
