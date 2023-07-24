use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, Email, NtRef, PhoneNumber, WebAddr},
    qb_object_data::QBObjectData,
};

/*
    Customer Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/customer
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
struct Customer {
    #[serde(flatten)]
    qb_data: QBObjectData,
    display_name: Option<String>,
    title: Option<String>,
    given_name: Option<String>,
    middle_name: Option<String>,
    suffix: Option<String>,
    family_name: Option<String>,
    primary_email_addr: Option<Email>,
    resale_num: Option<String>,
    secondary_tax_identifier: Option<String>,
    ar_account_ref: Option<NtRef>,
    default_tax_code_ref: Option<NtRef>,
    preferred_delivery_method: Option<String>,
    sales_term_ref: Option<NtRef>,
    customer_type_ref: Option<String>,
    fax: Option<PhoneNumber>,
    bill_with_parent: Option<bool>,
    currency_ref: Option<NtRef>,
    mobile: Option<PhoneNumber>,
    job: Option<bool>,
    balance_with_jobs: Option<f32>,
    primary_phone: Option<PhoneNumber>,
    open_balance_date: Option<NaiveDate>,
    taxable: Option<bool>,
    alternate_phone: Option<PhoneNumber>,
    parent_ref: Option<NtRef>,
    notes: Option<String>,
    web_addr: Option<WebAddr>,
    active: Option<bool>,
    company_name: Option<String>,
    balance: Option<f32>,
    ship_addr: Option<Addr>,
    payment_method_ref: Option<NtRef>,
    is_project: Option<bool>,
    source: Option<String>,
    print_check_on_name: Option<String>,
    bill_addr: Option<Addr>,
    fully_qualified_name: Option<String>,
    level: Option<u8>,
    tax_exemption_reason_id: Option<TaxExemptStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(from = "u8", into = "u8")]
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
    #[default] Other,
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
            _ => TaxExemptStatus::Other
        }
    }
}

impl Into<u8> for TaxExemptStatus {
    fn into(self) -> u8 {
        match self {
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
