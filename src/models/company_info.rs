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
/// https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companyinfo
pub struct CompanyInfo {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub company_addr: Option<Addr>,
    pub company_name: Option<String>,
    pub company_start_date: Option<NaiveDate>,
    pub country: Option<String>,
    pub customer_communication_addr: Option<Addr>,
    pub domain: Option<String>,
    pub email: Option<Email>,
    pub fiscal_year_start_month: Option<String>,
    pub legal_addr: Option<Addr>,
    pub legal_name: Option<String>,
    pub name_value: Option<Vec<NtRef>>,
    pub primary_phone: Option<PhoneNumber>,
    pub sparse: Option<bool>,
    pub supported_languages: Option<String>,
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
