use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{QBCreatable, QBFullUpdatable, QBReadable, QBToRef, QBSparseUpdateable};

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};

/*
    Customer Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/customer
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option), default))]
pub struct Customer {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub display_name: Option<String>,
    pub title: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    #[serde(rename="sparse")]
    pub sparse: Option<bool>,
    pub suffix: Option<String>,
    pub family_name: Option<String>,
    pub primary_email_addr: Option<Email>,
    pub resale_num: Option<String>,
    pub secondary_tax_identifier: Option<String>,
    pub ar_account_ref: Option<NtRef>,
    pub default_tax_code_ref: Option<NtRef>,
    pub preferred_delivery_method: Option<String>,
    pub sales_term_ref: Option<NtRef>,
    pub customer_type_ref: Option<String>,
    pub fax: Option<PhoneNumber>,
    pub bill_with_parent: Option<bool>,
    pub currency_ref: Option<NtRef>,
    pub mobile: Option<PhoneNumber>,
    pub job: Option<bool>,
    pub balance_with_jobs: Option<f32>,
    pub primary_phone: Option<PhoneNumber>,
    pub open_balance_date: Option<NaiveDate>,
    pub taxable: Option<bool>,
    pub alternate_phone: Option<PhoneNumber>,
    pub parent_ref: Option<NtRef>,
    pub notes: Option<String>,
    pub web_addr: Option<WebAddr>,
    pub active: Option<bool>,
    pub company_name: Option<String>,
    pub balance: Option<f32>,
    pub ship_addr: Option<Addr>,
    pub payment_method_ref: Option<NtRef>,
    pub is_project: Option<bool>,
    pub source: Option<String>,
    pub print_check_on_name: Option<String>,
    pub bill_addr: Option<Addr>,
    pub fully_qualified_name: Option<String>,
    pub level: Option<u8>,
    pub tax_exemption_reason_id: Option<TaxExemptStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(from = "u8", into = "u8")]
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
        self.can_full_update() && self.sparse.is_some_and(|x| x)
    }
}

impl QBToRef for Customer {
    fn ref_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }
}
