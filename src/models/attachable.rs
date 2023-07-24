use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{CustomField, MetaData, NtRef};

/*
    Attachable Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable
*/
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Attachable {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub meta_data: Option<MetaData>,
    pub file_name: Option<String>,
    pub note: Option<String>,
    pub category: Option<AttachmentCategory>,
    pub content_type: Option<String>,
    pub place_name: Option<String>,
    pub attachable_ref: Option<AttachableRef>,
    pub long: Option<String>,
    pub tag: Option<String>,
    pub lat: Option<String>,
    pub file_access_uri: Option<String>,
    pub size: Option<f32>,
    pub thumbnail_file_access_uri: Option<String>,
    pub temp_download_uri: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum AttachmentCategory {
    ContactPhoto,
    Document,
    Image,
    Receipt,
    Signature,
    Sound,
    #[default]
    Other,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct AttachableRef {
    pub include_on_send: Option<bool>,
    pub line_info: Option<String>,
    pub no_ref_only: Option<bool>,
    pub custom_field: Option<Vec<CustomField>>,
    pub inactive: Option<bool>,
    pub entity_ref: Option<NtRef>,
}
