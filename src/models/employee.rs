use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, PhoneNumber};

/*
    Employee object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/employee
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
struct Employee {
    id: Option<String>,
    sync_token: Option<String>,
    meta_data: Option<MetaData>,
    primary_addr: Option<Addr>,
    primary_email_addr: Option<Email>,
    display_name: Option<String>,
    title: Option<String>,
    billable_time: Option<bool>,
    given_name: Option<String>,
    birth_date: Option<NaiveDate>,
    middle_name: Option<String>,
    ssn: Option<String>,
    primary_phone: Option<PhoneNumber>,
    active: Option<bool>,
    released_date: Option<NaiveDate>,
    cost_rate: Option<f64>,
    mobile: Option<PhoneNumber>,
    gender: Option<String>,
    hired_date: Option<NaiveDate>,
    bill_rate: Option<f64>,
    organization: Option<bool>,
    suffix: Option<String>,
    family_name: Option<String>,
    print_on_check_name: Option<String>,
    employee_number: Option<String>,
    #[serde(rename = "V4IDPseudonym")]
    v4id_pseudonym: Option<String>,
}
