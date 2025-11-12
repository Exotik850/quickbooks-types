use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, PhoneNumber};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{QBCreatable, QBFullUpdatable, QBItem};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Employee
///
/// Represents an internal staff member or contractor tracked for payroll, time entry,
/// or billable activities in `QuickBooks` Online.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/employee>
pub struct Employee {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Primary address information for the employee
    pub primary_addr: Option<Addr>,
    /// Primary email address for the employee
    pub primary_email_addr: Option<Email>,
    /// Display name of the employee
    pub display_name: Option<String>,
    /// Title or position of the employee
    pub title: Option<String>,
    /// Indicates if the employee's time can be billed to customers
    pub billable_time: Option<bool>,
    /// The employee's first name
    pub given_name: Option<String>,
    /// The employee's birth date
    pub birth_date: Option<NaiveDate>,
    /// The employee's middle name
    pub middle_name: Option<String>,
    /// The employee's Social Security Number
    pub ssn: Option<String>,
    /// Primary phone number for the employee
    pub primary_phone: Option<PhoneNumber>,
    /// Indicates if the employee is active
    pub active: Option<bool>,
    /// Date when the employee was released from employment
    pub released_date: Option<NaiveDate>,
    /// The employee's cost rate per hour
    pub cost_rate: Option<f64>,
    /// Mobile phone number for the employee
    pub mobile: Option<PhoneNumber>,
    /// The employee's gender
    pub gender: Option<String>,
    /// Date when the employee was hired
    pub hired_date: Option<NaiveDate>,
    /// The rate at which the employee's time is billed to customers
    pub bill_rate: Option<f64>,
    /// Indicates if the employee is an organization rather than an individual
    pub organization: Option<bool>,
    /// The employee's name suffix (e.g. Jr, Sr, III)
    pub suffix: Option<String>,
    /// The employee's last name
    pub family_name: Option<String>,
    /// The employee's name as it should appear on checks
    pub print_on_check_name: Option<String>,
    /// The employee's identification number within the company
    pub employee_number: Option<String>,
    /// Identity provider pseudonym for the employee
    #[serde(rename = "V4IDPseudonym")]
    pub v4id_pseudonym: Option<String>,
}

impl QBCreatable for Employee {
    fn can_create(&self) -> bool {
        self.given_name.is_some() || self.family_name.is_some()
    }
}

impl QBFullUpdatable for Employee {
    fn can_full_update(&self) -> bool {
        self.has_read()
    }
}
