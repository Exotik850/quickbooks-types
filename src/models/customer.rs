use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Customer {
    id: String,
    sync_token: String,
    display_name: String,
    title: String,
    given_name: String,
    middle_name: String,
    suffix: String,
    family_name: String,
    primary_email_addr: Email,
    resale_num: String,
    secondary_tax_identifier: String,
    ar_account_ref: NtRef,
    default_tax_code_ref: NtRef,
    preferred_delivery_method: String,
    sales_term_ref: NtRef,
    customer_type_ref: String,
    fax: PhoneNumber,
    bill_with_parent: bool,
    currency_ref: NtRef,
    mobile: PhoneNumber,
    job: bool,
    balance_with_jobs: f32,
    primary_phone: PhoneNumber,
    open_balance_date: NaiveDate,
    taxable: bool,
    alternate_phone: PhoneNumber,
    meta_data: MetaData,
    parent_ref: NtRef,
    notes: String,
    web_addr: WebAddr,
    active: bool,
    company_name: String,
    balance: f32,
    ship_addr: Addr,
    payment_method_ref: NtRef,
    is_project: bool,
    source: String,
    print_check_on_name: String,
    bill_addr: Addr,
    fully_qualified_name: String,
    level: u8,
    tax_exemption_reason_id: Option<TaxExemptStatus>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(from="u8",into="u8")]
enum TaxExemptStatus {
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
            e => panic!("Unknown Tax Exempt Code: {e}"),
        }
    }
}

impl Into<u8> for TaxExemptStatus {
    fn into(self) -> u8 {
        match self {
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