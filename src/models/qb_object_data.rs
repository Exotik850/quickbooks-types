use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::MetaData;


#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct QBObjectData {
    id: Option<String>,
    sync_token: Option<String>,
    meta_data: Option<MetaData>,
}
