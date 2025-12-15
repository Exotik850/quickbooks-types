use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::{MetaData, NtRef};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
pub struct TaxCode {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the tax code
    pub name: Option<String>,
    /// Description of the tax code
    pub description: Option<String>,
    /// Indicates if the tax code is active
    pub active: Option<bool>,
    /// Indicates if the tax code is a sales tax code
    pub sales_tax_code: Option<bool>,
    /// List of purchase tax rates associated with the tax code
    pub purchase_tax_rate_list: Option<Vec<TaxRateDetail>>,
    /// List of sales tax rates associated with the tax code
    pub sales_tax_rate_list: Option<Vec<TaxRateDetail>>,
    /// Indicates if the tax code is a tax group
    pub tax_group: Option<bool>,
    /// Indicates if the tax code is taxable
    pub taxable: Option<bool>,
    /// Indicates if the tax code is hidden
    pub hidden: Option<bool>,
    /// Metadata about the entity
    pub meta_data: Option<MetaData>,
    /// Flag indicating if the tax code is system defined or user defined
    pub tax_code_config_type: Option<TaxCodeConfigType>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaxRateDetail {
    pub tax_rate_ref: NtRef,
    pub tax_type_applicable: Option<TaxTypeApplicable>,
    pub tax_order: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum TaxTypeApplicable {
    TaxOnAmount,
    TaxOnTax,
    TaxOnAmountPlusTax,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxCodeConfigType {
    UserDefined,
    SystemGenerated,
}
