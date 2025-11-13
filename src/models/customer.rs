use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{QBCreatable, QBFullUpdatable, QBReadable, QBSparseUpdateable};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Customer
///
/// Represents an individual or organization that purchases goods or services and is billed in `QuickBooks` Online.
///
/// Creation requirements:
/// - `QBCreatable::can_create()` returns true when at least one of these fields is present:
///   `display_name`, `given_name`, `family_name`, `middle_name`, `title`, or `suffix`.
///
/// Update semantics:
/// - `QBFullUpdatable::can_full_update()` requires `can_read()` (ID present) and `can_create()`.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/customer>
pub struct Customer {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Display name of the customer
    pub display_name: Option<String>,
    /// Title of the customer (e.g., Mr., Mrs., Ms.)
    pub title: Option<String>,
    /// First name of the customer
    pub given_name: Option<String>,
    /// Middle name of the customer
    pub middle_name: Option<String>,
    /// Indicates if the entity is a sparse object
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    /// Suffix of the customer's name (e.g., Jr., Sr., III)
    pub suffix: Option<String>,
    /// Last name of the customer
    pub family_name: Option<String>,
    /// Primary email address of the customer
    pub primary_email_addr: Option<Email>,
    /// Resale number for the customer
    pub resale_num: Option<String>,
    /// Secondary tax identifier for the customer
    pub secondary_tax_identifier: Option<String>,
    /// Reference to the Accounts Receivable account for the customer
    pub ar_account_ref: Option<NtRef>,
    /// Reference to the default tax code for the customer
    pub default_tax_code_ref: Option<NtRef>,
    /// Preferred delivery method for the customer (None, Print, Email, or Trax)
    pub preferred_delivery_method: Option<String>,
    /// Reference to the sales term for the customer
    pub sales_term_ref: Option<NtRef>,
    /// Reference to the customer type for the customer
    pub customer_type_ref: Option<String>,
    /// Fax number of the customer
    pub fax: Option<PhoneNumber>,
    /// Indicates if the customer is billed with their parent
    pub bill_with_parent: Option<bool>,
    /// Reference to the currency for the customer
    pub currency_ref: Option<NtRef>,
    /// Mobile phone number of the customer
    pub mobile: Option<PhoneNumber>,
    /// Indicates if the customer is a job
    pub job: Option<bool>,
    /// Balance including all jobs related to the customer
    pub balance_with_jobs: Option<f64>,
    /// Primary phone number of the customer
    pub primary_phone: Option<PhoneNumber>,
    /// Date of the open balance in YYYY-MM-DD format
    pub open_balance_date: Option<NaiveDate>,
    /// Indicates if the customer is taxable
    pub taxable: Option<bool>,
    /// Alternative phone number of the customer
    pub alternate_phone: Option<PhoneNumber>,
    /// Reference to the parent of the customer
    pub parent_ref: Option<NtRef>,
    /// Notes about the customer
    pub notes: Option<String>,
    /// Web address (URL) of the customer
    pub web_addr: Option<WebAddr>,
    /// Indicates if the customer is active
    pub active: Option<bool>,
    /// Company name of the customer
    pub company_name: Option<String>,
    /// Current balance of the customer
    pub balance: Option<f64>,
    /// Shipping address of the customer
    pub ship_addr: Option<Addr>,
    /// Reference to the default payment method for the customer
    pub payment_method_ref: Option<NtRef>,
    /// Indicates if the customer is a project
    pub is_project: Option<bool>,
    /// Source of the customer record
    ///
    /// DEPRECATED: as of 9/15/2025
    pub source: Option<String>,
    /// Name to print on checks for the customer
    pub print_check_on_name: Option<String>,
    /// Billing address of the customer
    pub bill_addr: Option<Addr>,
    /// Fully qualified name of the customer
    pub fully_qualified_name: Option<String>,
    /// Level in the customer hierarchy
    pub level: Option<String>,
    /// Tax exemption reason identifier for the customer
    pub tax_exemption_reason_id: Option<TaxExemptStatus>,
}

/// `TaxExemptStatus`
///
/// Enumerates QuickBooks-defined reason codes for tax exemption. Values may be returned
/// either as their numeric code (1-15) or as strings that can be parsed to those codes.
/// Unknown or unsupported values are coerced to `Other` during deserialization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[serde(into = "u8")]
pub enum TaxExemptStatus {
    FederalGovernment,
    StateGovernment,
    LocalGovernment,
    TribalGovernment,
    CharitableOrganization,
    ReligiousOrganization,
    EducationalOrganization,
    Hospital,
    Resale,
    DirectPayPermit,
    MultiplePointsOfUse,
    DirectMail,
    AgriculturalProduction,
    IndustrialProductionOrManufacturing,
    ForeignDiplomat,
    #[default]
    Other,
}

// TODO - Make sure this is necessary, as it might only be strings that are returned from
// the API, and not numbers.
impl<'de> Deserialize<'de> for TaxExemptStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrNumber {
            String(String),
            Number(u8),
        }

        match StringOrNumber::deserialize(deserializer)? {
            StringOrNumber::String(s) => match s.parse::<u8>() {
                Ok(num) => Ok(num.into()),
                Err(_) => Ok(TaxExemptStatus::Other),
            },
            StringOrNumber::Number(num) => Ok(num.into()),
        }
    }
}

impl From<u8> for TaxExemptStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => TaxExemptStatus::FederalGovernment,
            2 => TaxExemptStatus::StateGovernment,
            3 => TaxExemptStatus::LocalGovernment,
            4 => TaxExemptStatus::TribalGovernment,
            5 => TaxExemptStatus::CharitableOrganization,
            6 => TaxExemptStatus::ReligiousOrganization,
            7 => TaxExemptStatus::EducationalOrganization,
            8 => TaxExemptStatus::Hospital,
            9 => TaxExemptStatus::Resale,
            10 => TaxExemptStatus::DirectPayPermit,
            11 => TaxExemptStatus::MultiplePointsOfUse,
            12 => TaxExemptStatus::DirectMail,
            13 => TaxExemptStatus::AgriculturalProduction,
            14 => TaxExemptStatus::IndustrialProductionOrManufacturing,
            15 => TaxExemptStatus::ForeignDiplomat,
            _ => TaxExemptStatus::Other,
        }
    }
}

impl From<TaxExemptStatus> for u8 {
    fn from(value: TaxExemptStatus) -> u8 {
        match value {
            TaxExemptStatus::Other => 0,
            TaxExemptStatus::FederalGovernment => 1,
            TaxExemptStatus::StateGovernment => 2,
            TaxExemptStatus::LocalGovernment => 3,
            TaxExemptStatus::TribalGovernment => 4,
            TaxExemptStatus::CharitableOrganization => 5,
            TaxExemptStatus::ReligiousOrganization => 6,
            TaxExemptStatus::EducationalOrganization => 7,
            TaxExemptStatus::Hospital => 8,
            TaxExemptStatus::Resale => 9,
            TaxExemptStatus::DirectPayPermit => 10,
            TaxExemptStatus::MultiplePointsOfUse => 11,
            TaxExemptStatus::DirectMail => 12,
            TaxExemptStatus::AgriculturalProduction => 13,
            TaxExemptStatus::IndustrialProductionOrManufacturing => 14,
            TaxExemptStatus::ForeignDiplomat => 15,
        }
    }
}

impl QBCreatable for Customer {
    fn can_create(&self) -> bool {
        self.display_name.is_some()
            || self.suffix.is_some()
            || self.title.is_some()
            || self.middle_name.is_some()
            || self.family_name.is_some()
            || self.given_name.is_some()
    }
}

impl QBFullUpdatable for Customer {
    fn can_full_update(&self) -> bool {
        self.can_read() && self.can_create()
    }
}

impl QBSparseUpdateable for Customer {
    fn can_sparse_update(&self) -> bool {
        self.can_full_update()
    }
}
