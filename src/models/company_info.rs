use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, Email, MetaData, NtRef, PhoneNumber, WebAddr};
use crate::{QBFullUpdatable, QBSparseUpdateable};

#[cfg(feature = "builder")]
use crate::error::QBTypeError;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// CompanyInfo Object
///
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo>
/// Company Information
///
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo>
pub struct CompanyInfo {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Company address information
    pub company_addr: Option<Addr>,
    /// Name of the company
    pub company_name: Option<String>,
    /// Date the company was started in YYYY-MM-DD format
    pub company_start_date: Option<NaiveDate>,
    /// Country of the company
    pub country: Option<String>,
    /// Address used for customer communications
    pub customer_communication_addr: Option<Addr>,
    /// Domain of the company. Typically `QBO` for QuickBooks Online
    pub domain: Option<String>,
    /// Company email information
    pub email: Option<Email>,
    /// Month when the fiscal year starts
    pub fiscal_year_start_month: Option<String>,
    /// Legal address of the company
    pub legal_addr: Option<Addr>,
    /// Legal name of the company
    pub legal_name: Option<String>,
    /// Custom name-value pairs for the company
    pub name_value: Option<Vec<NtRef>>,
    /// Primary phone number for the company
    pub primary_phone: Option<PhoneNumber>,
    /// Indicates if the entity is a sparse object
    pub sparse: Option<bool>,
    /// Languages supported by the company
    pub supported_languages: Option<String>,
    /// Web address (website URL) of the company
    pub web_addr: Option<WebAddr>,
}

impl QBFullUpdatable for CompanyInfo {
    fn can_full_update(&self) -> bool {
        self.sync_token.is_some() && self.company_name.is_some() && self.company_addr.is_some()
    }
}

impl QBSparseUpdateable for CompanyInfo {
    fn can_sparse_update(&self) -> bool {
        self.can_full_update() && self.sparse.is_some_and(|x| x)
    }
}
