use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
pub struct OffsetAndLimit<SortBy> {
    #[serde(rename = "i")]
    #[schemars(title = "offset")]
    pub offset: Option<i32>,
    #[serde(rename = "n")]
    #[schemars(title = "limit")]
    pub limit: Option<i32>,
    #[serde(rename = "k")]
    #[schemars(title = "sort_by")]
    pub sort_by: Option<SortBy>,
}
