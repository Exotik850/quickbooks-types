use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, Email, NtRef, PhoneNumber, WebAddr},
    qb_object_data::QBObjectData,
};

/*
    CompanyInfo Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct CompanyInfo {
    #[serde(flatten)]
    qb_data: QBObjectData,
    pub company_addr: Option<Addr>,
    pub company_name: Option<String>,
    pub company_start_date: Option<NaiveDate>,
    pub country: Option<String>,
    pub customer_communication_addr: Option<Addr>,
    pub domain: Option<String>,
    pub email: Option<Email>,
    pub fiscal_year_start_month: Option<String>,
    pub legal_addr: Option<Addr>,
    pub legal_name: Option<String>,
    pub name_value: Option<Vec<NtRef>>,
    pub primary_phone: Option<PhoneNumber>,
    pub sparse: Option<bool>,
    pub supported_languages: Option<String>,
    pub web_addr: Option<WebAddr>,
}
