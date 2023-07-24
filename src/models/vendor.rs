use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{qb_object_data::QBObjectData, common::{Email, PhoneNumber, NtRef, WebAddr, Addr}};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct Vendor {
    #[serde(flatten)]
    qb_data: QBObjectData,
    title: Option<String>,
    given_name: Option<String>,
    middle_name: Option<String>,
    suffix: Option<String>,
    family_name: Option<String>,
    primary_email_addr: Option<Email>,
    display_name: Option<String>,
    other_contact_info: Option<ContactInfo>,
    #[serde(rename="APAccountRef")]
    ap_account_ref: Option<NtRef>,
    term_ref: Option<NtRef>,
    source: Option<String>,
    #[serde(rename="GSTIN")]
    gstin: Option<String>,
    #[serde(rename="T4AEligible")]
    t4a_eligible: Option<bool>,
    fax: Option<PhoneNumber>,
    business_number: Option<String>,
    currency_ref: Option<NtRef>,
    #[serde(rename="HasTPAR")]
    has_tpar: Option<bool>,
    tax_reporting_basis: Option<String>,
    mobile: Option<PhoneNumber>,
    primary_phone: Option<PhoneNumber>,
    active: Option<bool>,
    alternate_phone: Option<PhoneNumber>,
    vendor_1099: Option<bool>,
    cost_rate: Option<f64>,
    bill_rate: Option<f32>,
    web_addr: Option<WebAddr>,
    t5018_eligible: Option<bool>,
    company_name: Option<String>,
    vendor_payment_bank_detail: Option<VendorPaymentBankDetail>,
    tax_identifier: Option<String>,
    acct_num: Option<String>,
    #[serde(rename="GSTRegistrationType")]
    gst_registration_type: Option<String>,
    print_check_on_name: Option<String>,
    bill_addr: Option<Addr>,
    balance: Option<f32>
}

// Weird type they used for just this specific object
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct ContactInfo {
    #[serde(rename="Type")]
    contact_type: Option<String>,
    telephone: Option<PhoneNumber>,
}

// Another one
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct VendorPaymentBankDetail {
    bank_account_name: Option<String>,
    bank_branch_identifier: Option<String>,
    bank_account_number: Option<String>,
    statement_text: Option<String>,
}