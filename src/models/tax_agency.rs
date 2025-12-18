use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::MetaData, QBCreatable};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `TaxAgency`
///
/// Represents a tax agency responsible for collecting and remitting taxes.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/taxagency>
pub struct TaxAgency {
    /// The unique ID of the tax agency
    pub id: Option<String>,
    /// Name of the tax agency
    pub display_name: Option<String>,
    /// The unique sync token of the tax agency, used for concurrency control
    pub sync_token: Option<String>,
    /// Indicates if the tax agency is active
    pub tax_tracked_on_sales: Option<bool>,
    /// Indicates if the tax agency is tracked on purchases
    pub tax_tracked_on_purchases: Option<bool>,
    /// Tax registration number of the tax agency
    pub tax_registration_number: Option<String>,
    /// Metadata about the tax agency
    pub meta_data: Option<MetaData>,
    /// Date of the last file submitted to the tax agency
    pub last_file_date: Option<NaiveDate>,
    /// Configuration of the tax agency
    pub tax_agency_config: Option<TaxAgencyConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Configuration options for the tax agency
pub enum TaxAgencyConfig {
    UserDefined,
    SystemGenerated,
}

impl QBCreatable for TaxAgency {
    fn can_create(&self) -> bool {
        self.display_name
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty())
    }
}
