use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyInfo {
    #[serde(default)]
    pub company_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub company_name: String,

    pub company_start_date: NaiveDate,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(default)]
    pub customer_communication_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default)]
    pub email: Email,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fiscal_year_start_month: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub legal_addr: Addr,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub legal_name: String,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name_value: Vec<NtRef>,
    #[serde(default)]
    pub primary_phone: PhoneNumber,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub supported_languages: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub web_addr: WebAddr,
}
