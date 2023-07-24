use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{CustomField, MetaData, NtRef};

/*
    Attachable Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable
*/
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
struct Attachable {
    id: Option<String>,
    sync_token: Option<String>,
    meta_data: Option<MetaData>,
    file_name: Option<String>,
    note: Option<String>,
    category: Option<AttachmentCategory>,
    content_type: Option<String>,
    place_name: Option<String>,
    attachable_ref: Option<AttachableRef>,
    long: Option<String>,
    tag: Option<String>,
    lat: Option<String>,
    file_access_uri: Option<String>,
    size: Option<f32>,
    thumbnail_file_access_uri: Option<String>,
    temp_download_uri: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
enum AttachmentCategory {
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
struct AttachableRef {
    include_on_send: Option<bool>,
    line_info: Option<String>,
    no_ref_only: Option<bool>,
    custom_field: Option<Vec<CustomField>>,
    inactive: Option<bool>,
    entity_ref: Option<NtRef>,
}
