use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::MetaData, QBFullUpdatable};

/*
    Preferences Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/preferences 
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into), default))]
pub struct Preferences {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub email_message_prefs: Option<EmailMessagePrefs>,
    pub product_and_services_prefs: Option<ProductAndServicesPrefs>,
    pub report_prefs: Option<ReportPrefs>,
    pub accounting_info_prefs: Option<AccountingInfoPrefs>,
    pub sales_forms_prefs: Option<SalesFormsPrefs>,
    pub vendor_and_purchases_prefs: Option<VendorAndPurchasesPrefs>,
    pub tax_prefs: Option<TaxPrefs>,
    pub other_prefs: Option<OtherPrefs>,
    pub time_tracking_prefs: Option<TimeTrackingPrefs>,
    pub currency_prefs: Option<CurrencyPrefs>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct EmailMessagePrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct ProductAndServicesPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct ReportPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct AccountingInfoPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct SalesFormsPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct VendorAndPurchasesPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct TaxPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct OtherPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct TimeTrackingPrefs;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct CurrencyPrefs;

impl QBFullUpdatable for Preferences {
    fn can_full_update(&self) -> bool {
        self.id.is_some() 
        && self.sync_token.is_some()
    }
}