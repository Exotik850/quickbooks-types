use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{CustomField, MetaData},
    QBCreatable, QBFullUpdatable, QBItem,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `CompanyCurrency`
///
/// Represents a currency used in transactions.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/companycurrency>
pub struct CompanyCurrency {
    /// The unique ID of the currency
    pub id: Option<String>,
    /// ISO 4217 currency code
    ///
    /// TODO: Make an enum based on <https://developer.intuit.com/app/developer/qbo/docs/workflows/manage-multiple-currencies>
    pub code: Option<String>,
    /// The unique sync token of the currency, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the currency
    pub name: Option<String>,
    /// Custom fields associated with the currency
    pub custom_field: Option<Vec<CustomField>>,
    /// Indicates if the currency is active
    pub active: Option<bool>,
    /// Metadata about the currency
    pub meta_data: Option<MetaData>,
}

impl QBCreatable for CompanyCurrency {
    fn can_create(&self) -> bool {
        self.code.is_some()
    }
}

impl QBFullUpdatable for CompanyCurrency {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}
