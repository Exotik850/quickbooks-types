use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{Email, MetaData, NtRef}, QBError, QBFullUpdatable, QBItem
};

/*
    Preferences Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/preferences
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBError"), setter(into, strip_option))
)]

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

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct EmailMessagePrefs {
    pub invoice_message: Option<EmailMessageType>,
    pub estimate_message: Option<EmailMessageType>,
    pub sales_receipt_message: Option<EmailMessageType>,
    pub statement_message: Option<EmailMessageType>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct EmailMessageType {
    pub message: Option<String>,
    pub subject: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ProductAndServicesPrefs {
    pub revenue_recognition_enabled: Option<bool>,
    pub recognition_frequency_type: Option<String>,
    pub for_sales: Option<bool>,
    pub quantity_on_hand: Option<bool>,
    pub quantity_with_price_and_rate: Option<bool>,
    pub for_purchase: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ReportPrefs {
    pub report_basis: Option<String>,
    pub calc_aging_report_from_txn_date: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct AccountingInfoPrefs {
    pub first_month_of_fiscal_year: Option<String>,
    pub use_account_numbers: Option<bool>,
    pub tax_year_month: Option<String>,
    pub class_tracking_per_txn: Option<bool>,
    pub track_departments: Option<bool>,
    pub tax_form: Option<String>,
    pub customer_terminology: Option<String>,
    pub book_close_date: Option<NaiveDate>,
    pub department_terminology: Option<String>,
    pub class_tracking_per_txn_line: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SalesFormsPrefs {
    pub sales_emaill_bcc: Option<Email>,
    pub sales_email_cc: Option<Email>,
    pub using_progress_invoicing: Option<bool>,
    pub custom_field: Option<String>, // TODO
    pub allow_service_date: Option<bool>,
    pub estimate_message: Option<String>,
    pub email_copy_to_company: Option<bool>,
    pub default_customer_message: Option<String>,
    pub allow_shipping: Option<bool>,
    pub default_discount_account: Option<bool>,
    #[serde(rename = "IPNSupportEnabled")]
    pub ipn_support_enabled: Option<bool>,
    pub e_transaction_payment_enabled: Option<bool>,
    pub default_terms: Option<NtRef>,
    pub allow_deposit: Option<bool>,
    pub using_price_levels: Option<bool>,
    pub default_shipping_account: Option<bool>,
    #[serde(rename = "ETransactionAttachPDF")]
    pub e_transaction_attach_pdf: Option<bool>,
    pub custom_txn_numbers: Option<bool>,
    pub e_transaction_enabled_status: Option<String>,
    pub allow_estimates: Option<bool>,
    pub allow_discount: Option<bool>,
    pub auto_apply_credit: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct VendorAndPurchasesPrefs {
    #[serde(rename = "POCustomField")]
    pub po_custom_field: Option<String>, // TODO
    pub default_markup_account: Option<NtRef>,
    pub tracking_by_customer: Option<bool>,
    pub default_terms: Option<NtRef>,
    pub billable_expense_tracking: Option<bool>,
    pub default_markup: Option<f32>,
    #[serde(rename = "TPAREnabled")]
    pub tpar_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TaxPrefs {
    pub partner_tax_enabled: Option<bool>,
    pub tax_group_code_ref: Option<String>,
    pub using_sales_tax: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct OtherPrefs {
    pub name_value: Option<Vec<NtRef>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TimeTrackingPrefs {
    pub work_week_start_date: Option<String>,
    pub mark_time_entries_billable: Option<bool>,
    pub show_bill_rate_to_all: Option<bool>,
    pub using_sales_tax: Option<bool>,
    pub bill_customers: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CurrencyPrefs {
    pub home_currency: Option<NtRef>,
    pub multi_currency_enabled: Option<bool>,
}

impl QBFullUpdatable for Preferences {
    fn can_full_update(&self) -> bool {
        self.has_read()
    }
}
