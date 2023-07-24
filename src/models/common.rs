use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Line;

/*
    These are not full quickbooks object but they are used in other quickbooks objects,
    they have no documentation of their own but their types are shown in the objects
    they are used in
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct NtRef {
    // Reference Type
    #[serde(rename = "type")]
    pub entity_ref_type: Option<String>,
    #[serde(alias = "Name")]
    pub name: Option<String>,
    #[serde(alias = "Value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MetaData {
    pub create_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Email {
    pub address: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Addr {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_sub_division_code: Option<String>,
    pub id: Option<String>,
    pub line1: Option<String>,
    pub postal_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebAddr {
    #[serde(default, rename = "URL")]
    url: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct PhoneNumber {
    pub free_form_number: Option<String>,
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {} {}",
            self.line1.as_ref().unwrap_or(&"".to_owned()),
            self.city.as_ref().unwrap_or(&"".to_owned()),
            self.country_sub_division_code
                .as_ref()
                .unwrap_or(&"".to_owned()),
            self.country.as_ref().unwrap_or(&"".to_owned()),
            self.postal_code.as_ref().unwrap_or(&"".to_owned())
        )
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct LinkedTxn {
    pub txn_id: Option<String>,
    pub txn_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct CustomField {
    definition_id: Option<String>,
    string_value: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    field_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct MarkupInfo {
    percent_based: Option<bool>,
    value: Option<f32>,
    percent: Option<f32>,
    price_level_ref: Option<NtRef>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct TxnTaxDetail {
    txn_tax_code_ref: Option<NtRef>,
    total_tax: Option<f32>,
    tax_line: Option<Vec<Line>>,
}
