use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct NtRef {
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub entity_ref_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty", alias = "Name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty", alias = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MetaData {
    pub create_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub address: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Addr {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country_sub_division_code: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub line1: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub postal_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebAddr {
    #[serde(default, rename = "URL", skip_serializing_if = "String::is_empty")]
    url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhoneNumber {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub free_form_number: String,
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{street_addr}, {city}, {country_sub_division_code}, {country} {postal_code}",
            street_addr = self.line1,
            city = self.city,
            country_sub_division_code = self.country_sub_division_code,
            country = self.country,
            postal_code = self.postal_code
        )
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct LinkedTxn {
    pub txn_id: Option<String>,
    pub txn_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct CustomField {
    definition_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkupInfo {
    percent_based: bool,
    value: f32,
    percent: f32, 
    price_level_ref: NtRef,
}