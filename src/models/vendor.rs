use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::QBCreatable;

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option), default))]
pub struct Vendor {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub meta_data: Option<MetaData>,
    pub title: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub family_name: Option<String>,
    pub primary_email_addr: Option<Email>,
    pub display_name: Option<String>,
    pub other_contact_info: Option<ContactInfo>,
    #[serde(rename = "APAccountRef")]
    pub ap_account_ref: Option<NtRef>,
    pub term_ref: Option<NtRef>,
    pub source: Option<String>,
    #[serde(rename = "GSTIN")]
    pub gstin: Option<String>,
    #[serde(rename = "T4AEligible")]
    pub t4a_eligible: Option<bool>,
    pub fax: Option<PhoneNumber>,
    pub business_number: Option<String>,
    pub currency_ref: Option<NtRef>,
    #[serde(rename = "HasTPAR")]
    pub has_tpar: Option<bool>,
    pub tax_reporting_basis: Option<String>,
    pub mobile: Option<PhoneNumber>,
    pub primary_phone: Option<PhoneNumber>,
    pub active: Option<bool>,
    pub alternate_phone: Option<PhoneNumber>,
    pub vendor_1099: Option<bool>,
    pub cost_rate: Option<f64>,
    pub bill_rate: Option<f32>,
    pub web_addr: Option<WebAddr>,
    pub t5018_eligible: Option<bool>,
    pub company_name: Option<String>,
    pub vendor_payment_bank_detail: Option<VendorPaymentBankDetail>,
    pub tax_identifier: Option<String>,
    pub acct_num: Option<String>,
    #[serde(rename = "GSTRegistrationType")]
    pub gst_registration_type: Option<String>,
    pub print_check_on_name: Option<String>,
    pub bill_addr: Option<Addr>,
    pub balance: Option<f32>,
}

// Weird type they used for just this specific object
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option), default))]
pub struct ContactInfo {
    #[serde(rename = "Type")]
    contact_type: Option<String>,
    telephone: Option<PhoneNumber>,
}

// Another one
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option), default))]
pub struct VendorPaymentBankDetail {
    bank_account_name: Option<String>,
    bank_branch_identifier: Option<String>,
    bank_account_number: Option<String>,
    statement_text: Option<String>,
}

impl QBCreatable for Vendor {
    fn can_create(&self) -> bool {
        self.display_name.is_some()
            || self.suffix.is_some()
            || self.title.is_some()
            || self.middle_name.is_some()
            || self.family_name.is_some()
            || self.given_name.is_some()
    }
}
