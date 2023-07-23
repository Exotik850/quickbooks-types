use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::MetaData;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct QBObjectData {
    id: Option<String>,
    sync_token: Option<String>,
    meta_data: Option<MetaData>,
}
