use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineField;

/*
    These are not full quickbooks object but they are used in other quickbooks objects,
    they have no documentation of their own but their types are shown in the objects
    they are used in
*/

/// Type used to hold ID's and/or references to other QB
/// Objects
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(default)]
pub struct NtRef {
    #[serde(rename = "type")]
    pub entity_ref_type: Option<String>,
    #[serde(alias = "Name")]
    pub name: Option<String>,
    #[serde(alias = "Value")]
    pub value: Option<String>,
}

impl From<&str> for NtRef {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value.into()),
            ..Default::default()
        }
    }
}

impl From<(&str, &str)> for NtRef {
    fn from(value: (&str, &str)) -> Self {
        Self {
            name: Some(value.0.into()),
            value: Some(value.1.into()),
            ..Default::default()
        }
    }
}

impl From<String> for NtRef {
    fn from(value: String) -> Self {
        Self {
            value: Some(value),
            ..Default::default()
        }
    }
}

impl From<(String, String)> for NtRef {
    fn from(value: (String, String)) -> Self {
        let (name, value) = value;
        Self {
            name: Some(name),
            value: Some(value),
            ..Default::default()
        }
    }
}

/// MetaData
/// 
/// Metadata about the transaction
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MetaData {
    pub create_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Email {
    pub address: Option<String>,
}

/// Address information
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Addr {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_sub_division_code: Option<String>,
    pub id: Option<String>,
    pub line1: Option<String>,
    pub postal_code: Option<String>,
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {} {}",
            self.line1.as_ref().unwrap_or(&String::new()),
            self.city.as_ref().unwrap_or(&String::new()),
            self.country_sub_division_code
                .as_ref()
                .unwrap_or(&String::new()),
            self.country.as_ref().unwrap_or(&String::new()),
            self.postal_code.as_ref().unwrap_or(&String::new())
        )
    }
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
pub struct PhoneNumber {
    pub free_form_number: Option<String>,
}

/// Linked Transaction
/// 
/// Information about a linked transaction.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct LinkedTxn {
    pub txn_id: Option<String>,
    pub txn_type: Option<String>,
}

/// CustomField
/// 
/// Custom field information
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CustomField {
    pub definition_id: Option<String>,
    pub string_value: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct MarkupInfo {
    pub percent_based: Option<bool>,
    pub value: Option<f64>,
    pub percent: Option<f64>,
    pub price_level_ref: Option<NtRef>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TxnTaxDetail {
    pub txn_tax_code_ref: Option<NtRef>,
    pub total_tax: Option<f64>,
    pub tax_line: Option<LineField>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct DeliveryInfo {
    pub delivery_type: String,
    pub delivery_time: DateTime<Utc>,
}

/// CreditCardPayment
/// 
/// Information about a credit card payment for the transaction.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CreditCardPayment {
    pub credit_charge_response: Option<CreditChargeResponse>,
    pub credit_charge_info: Option<CreditChargeInfo>,
}

/// CreditChargeResponse
/// 
/// Information about a credit charge for a transaction.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CreditChargeResponse {
    pub status: Option<CCPaymentStatus>,
    pub auth_code: Option<String>,
    pub txn_authorization_time: Option<DateTime<Utc>>,
    #[serde(rename = "CCTransId")]
    pub cc_trans_id: Option<String>,
}

/// CCPaymentStatus Enum
/// 
/// Status of the credit card payment
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum CCPaymentStatus {
    Completed,
    #[default]
    Unkown,
}

/// CreditChargeInfo
/// 
/// Information about a credit card payment for the transaction.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CreditChargeInfo {
    cc_expiry_month: Option<u32>,
    process_payment: Option<bool>,
    postal_code: Option<String>,
    amount: Option<f64>,
    name_on_acct: Option<String>,
    cc_expiry_year: Option<u32>,
    #[serde(rename = "type")]
    card_type: Option<String>,
    bill_addr_street: Option<String>,
}

/// PrintStatus Enum
/// 
/// Status of whether a document needs to be printed or not
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum PrintStatus {
    #[default]
    NotSet,
    NeedToPrint,
    PrintComplete,
}

/// EmailStatus Enum
/// 
/// Status of the email
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum EmailStatus {
    #[default]
    NotSet,
    NotSent,
    NeedToSend,
    EmailSent,
}

/// GlobalTaxCalculation Enum
/// 
/// Method in which tax is applied
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum GlobalTaxCalculation {
    WithinFrance,
    FranceOverseas,
    OutsideFranceWithEU,
    OutsideEU,
    #[default]
    #[serde(skip)]
    Other,
}
