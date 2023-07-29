use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, PhoneNumber};

/*
    Employee object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/employee
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Employee {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub meta_data: Option<MetaData>,
    pub primary_addr: Option<Addr>,
    pub primary_email_addr: Option<Email>,
    pub display_name: Option<String>,
    pub title: Option<String>,
    pub billable_time: Option<bool>,
    pub given_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub middle_name: Option<String>,
    pub ssn: Option<String>,
    pub primary_phone: Option<PhoneNumber>,
    pub active: Option<bool>,
    pub released_date: Option<NaiveDate>,
    pub cost_rate: Option<f64>,
    pub mobile: Option<PhoneNumber>,
    pub gender: Option<String>,
    pub hired_date: Option<NaiveDate>,
    pub bill_rate: Option<f64>,
    pub organization: Option<bool>,
    pub suffix: Option<String>,
    pub family_name: Option<String>,
    pub print_on_check_name: Option<String>,
    pub employee_number: Option<String>,
    #[serde(rename = "V4IDPseudonym")]
    pub v4id_pseudonym: Option<String>,
}