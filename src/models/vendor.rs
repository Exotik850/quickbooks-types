use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};
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
/// Vendor
///
/// Represents a supplier/payee from whom goods or services are purchased and to whom bills are owed in `QuickBooks` Online.
///
/// Creation requirements:
/// - `QBCreatable::can_create()` returns true when at least one of the following is present:
///   `display_name`, `given_name`, `family_name`, `middle_name`, `title`, or `suffix`.
///
/// Update semantics:
/// - `QBFullUpdatable::can_full_update()` returns true when both `has_read()` (ID + sync token are set)
///   and `can_create()` are true.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/vendor>
pub struct Vendor {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Title of the vendor
    pub title: Option<String>,
    /// Given name of the vendor
    pub given_name: Option<String>,
    /// Middle name of the vendor
    pub middle_name: Option<String>,
    /// Suffix of the vendor's name
    pub suffix: Option<String>,
    /// Family name of the vendor
    pub family_name: Option<String>,
    /// Primary email address of the vendor
    pub primary_email_addr: Option<Email>,
    /// Display name of the vendor
    pub display_name: Option<String>,
    /// Other contact information for the vendor
    pub other_contact_info: Option<ContactInfo>,
    /// Accounts Payable account reference
    #[serde(rename = "APAccountRef")]
    pub ap_account_ref: Option<NtRef>,
    /// Term reference for the vendor
    pub term_ref: Option<NtRef>,
    /// Source of the vendor information
    ///
    /// DEPRECATED: as of 9/15/2025
    pub source: Option<String>,
    /// GSTIN of the vendor
    #[serde(rename = "GSTIN")]
    pub gstin: Option<String>,
    /// Indicates if the vendor is T4A eligible
    #[serde(rename = "T4AEligible")]
    pub t4a_eligible: Option<bool>,
    /// Fax number of the vendor
    pub fax: Option<PhoneNumber>,
    /// Business number of the vendor
    pub business_number: Option<String>,
    /// Currency reference for the vendor
    pub currency_ref: Option<NtRef>,
    /// Indicates if the vendor has TPAR
    #[serde(rename = "HasTPAR")]
    pub has_tpar: Option<bool>,
    /// Tax reporting basis for the vendor
    pub tax_reporting_basis: Option<String>,
    /// Mobile phone number of the vendor
    pub mobile: Option<PhoneNumber>,
    /// Primary phone number of the vendor
    pub primary_phone: Option<PhoneNumber>,
    /// Indicates if the vendor is active
    pub active: Option<bool>,
    /// Alternate phone number of the vendor
    pub alternate_phone: Option<PhoneNumber>,
    /// Indicates if the vendor is 1099 eligible
    pub vendor_1099: Option<bool>,
    /// Cost rate for the vendor
    pub cost_rate: Option<f64>,
    /// Bill rate for the vendor
    pub bill_rate: Option<f64>,
    /// Web address of the vendor
    pub web_addr: Option<WebAddr>,
    /// Indicates if the vendor is T5018 eligible
    pub t5018_eligible: Option<bool>,
    /// Company name of the vendor
    pub company_name: Option<String>,
    /// Bank details for vendor payment
    pub vendor_payment_bank_detail: Option<VendorPaymentBankDetail>,
    /// Tax identifier for the vendor
    pub tax_identifier: Option<String>,
    /// Account number for the vendor
    pub acct_num: Option<String>,
    /// GST registration type for the vendor
    #[serde(rename = "GSTRegistrationType")]
    pub gst_registration_type: Option<String>,
    /// Name to print on checks for the vendor
    pub print_check_on_name: Option<String>,
    /// Billing address of the vendor
    pub bill_addr: Option<Addr>,
    /// Balance for the vendor
    pub balance: Option<f64>,
}

/// Contact Information
///
/// Represents additional contact information for a vendor in `QuickBooks` Online.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ContactInfo {
    #[serde(rename = "Type")]
    contact_type: Option<String>,
    telephone: Option<PhoneNumber>,
}

/// Vendor Payment Bank Detail
///
/// Represents the bank details used for vendor payments in `QuickBooks` Online.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
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

impl QBFullUpdatable for Vendor {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}
