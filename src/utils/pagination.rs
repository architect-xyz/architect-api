use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
pub struct OffsetAndLimit<SortBy> {
    #[serde(rename = "i")]
    pub offset: Option<i32>,
    #[serde(rename = "n")]
    pub limit: Option<i32>,
    #[serde(rename = "k")]
    pub sort_by: Option<SortBy>,
}
