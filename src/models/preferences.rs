use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{
    common::{Email, MetaData, NtRef, TypedRef},
    impl_linked, Account, CompanyCurrency, QBFullUpdatable, QBItem, Term,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Preferences
///
/// Company-wide configuration controlling behavior for sales forms,
/// purchasing, accounting, taxes, time tracking, currencies, and messaging
/// in `QuickBooks` Online.
///
/// Semantics:
/// - Models data only; no HTTP calls are performed in this crate.
/// - `QBFullUpdatable::can_full_update()` returns true when `has_read()` is true
///   (both `id` and `sync_token` are present).
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/preferences>
pub struct Preferences {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Preferences for email messages
    pub email_message_prefs: Option<EmailMessagePrefs>,
    /// Preferences for products and services
    pub product_and_services_prefs: Option<ProductAndServicesPrefs>,
    /// Preferences for reports
    pub report_prefs: Option<ReportPrefs>,
    /// Preferences for accounting information
    pub accounting_info_prefs: Option<AccountingInfoPrefs>,
    /// Preferences for sales forms
    pub sales_forms_prefs: Option<SalesFormsPrefs>,
    /// Preferences for vendors and purchases
    pub vendor_and_purchases_prefs: Option<VendorAndPurchasesPrefs>,
    /// Preferences for taxes
    pub tax_prefs: Option<TaxPrefs>,
    /// Other miscellaneous preferences
    pub other_prefs: Option<OtherPrefs>,
    /// Preferences for time tracking
    pub time_tracking_prefs: Option<TimeTrackingPrefs>,
    /// Preferences for currency
    pub currency_prefs: Option<CurrencyPrefs>,
}

/// Email Message Preferences
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct EmailMessagePrefs {
    pub invoice_message: Option<EmailMessageType>,
    pub estimate_message: Option<EmailMessageType>,
    pub sales_receipt_message: Option<EmailMessageType>,
    pub statement_message: Option<EmailMessageType>,
}

/// Email Message Type
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct EmailMessageType {
    pub message: Option<String>,
    pub subject: Option<String>,
}

/// Product and Services Preferences
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

/// Report Preferences
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ReportPrefs {
    pub report_basis: Option<String>,
    pub calc_aging_report_from_txn_date: Option<bool>,
}

/// Accounting Info Preferences
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
/// Sales Forms Preferences
///
/// Represents the preferences for sales forms in `QuickBooks`.
pub struct SalesFormsPrefs {
    /// BCC email address for sales forms.
    pub sales_emaill_bcc: Option<Email>,
    /// CC email address for sales forms.
    pub sales_email_cc: Option<Email>,
    /// Indicates if progress invoicing is being used.
    pub using_progress_invoicing: Option<bool>,
    /// Custom field for sales forms.
    pub custom_field: Option<String>, // TODO
    /// Indicates if service date is allowed on sales forms.
    pub allow_service_date: Option<bool>,
    /// Default message for estimates.
    pub estimate_message: Option<String>,
    /// Indicates if a copy of the email should be sent to the company.
    pub email_copy_to_company: Option<bool>,
    /// Default customer message for sales forms.
    pub default_customer_message: Option<String>,
    /// Indicates if shipping is allowed on sales forms.
    pub allow_shipping: Option<bool>,
    /// Indicates if the default discount account is enabled.
    pub default_discount_account: Option<bool>,
    /// Indicates if IPN support is enabled.
    #[serde(rename = "IPNSupportEnabled")]
    pub ipn_support_enabled: Option<bool>,
    /// Indicates if e-transaction payment is enabled.
    pub e_transaction_payment_enabled: Option<bool>,
    /// Default terms for sales forms.
    pub default_terms: Option<TypedRef<Term>>,
    /// Indicates if deposits are allowed on sales forms.
    pub allow_deposit: Option<bool>,
    /// Indicates if price levels are being used.
    pub using_price_levels: Option<bool>,
    /// Indicates if the default shipping account is enabled.
    pub default_shipping_account: Option<bool>,
    /// Indicates if e-transaction PDF attachment is enabled.
    #[serde(rename = "ETransactionAttachPDF")]
    pub e_transaction_attach_pdf: Option<bool>,
    /// Indicates if custom transaction numbers are allowed.
    pub custom_txn_numbers: Option<bool>,
    /// Status of e-transaction enabled.
    pub e_transaction_enabled_status: Option<String>,
    /// Indicates if estimates are allowed.
    pub allow_estimates: Option<bool>,
    /// Indicates if discounts are allowed.
    pub allow_discount: Option<bool>,
    /// Indicates if auto-apply credit is enabled.
    pub auto_apply_credit: Option<bool>,
}

impl_linked!(SalesFormsPrefs as default_terms => Term);

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
/// Vendor and Purchases Preferences
///
/// Represents the preferences related to vendors and purchases.
pub struct VendorAndPurchasesPrefs {
    /// Custom field for purchase orders
    #[serde(rename = "POCustomField")]
    pub po_custom_field: Option<String>, // TODO
    /// Reference to the default markup account
    pub default_markup_account: Option<TypedRef<Account>>,
    /// Indicates if tracking by customer is enabled
    pub tracking_by_customer: Option<bool>,
    /// Reference to the default terms
    pub default_terms: Option<TypedRef<Term>>,
    /// Indicates if billable expense tracking is enabled
    pub billable_expense_tracking: Option<bool>,
    /// Default markup value
    pub default_markup: Option<f64>,
    /// Indicates if TPAR (Taxable Payments Annual Report) is enabled
    #[serde(rename = "TPAREnabled")]
    pub tpar_enabled: Option<bool>,
}

impl_linked!(VendorAndPurchasesPrefs as default_markup_account => Account);
impl_linked!(VendorAndPurchasesPrefs as default_terms => Term);

/// Tax Preferences
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TaxPrefs {
    pub partner_tax_enabled: Option<bool>,
    pub tax_group_code_ref: Option<String>,
    pub using_sales_tax: Option<bool>,
}

/// Miscellaneous Other Preferences
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct OtherPrefs {
    /// Name value pairs for miscellaneous preferences
    pub name_value: Option<Vec<NtRef>>,
    // Capture everything else into a value map
    //
    // TODO: Handle specific fields as needed
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
/// Time Tracking Preferences
///
/// Represents the preferences for time tracking in `QuickBooks`.
pub struct TimeTrackingPrefs {
    /// The start date of the work week.
    pub work_week_start_date: Option<String>,
    /// Indicates if time entries should be marked as billable.
    pub mark_time_entries_billable: Option<bool>,
    /// Indicates if the bill rate should be shown to all users.
    pub show_bill_rate_to_all: Option<bool>,
    /// Indicates if sales tax is being used.
    pub using_sales_tax: Option<bool>,
    /// Indicates if customers should be billed.
    pub bill_customers: Option<bool>,
}

/// Currency Preferences
///
/// Represents the preferences related to currency handling in `QuickBooks`.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CurrencyPrefs {
    pub home_currency: Option<TypedRef<CompanyCurrency>>,
    pub multi_currency_enabled: Option<bool>,
}

impl_linked!(CurrencyPrefs as home_currency => CompanyCurrency);

impl QBFullUpdatable for Preferences {
    fn can_full_update(&self) -> bool {
        self.has_read()
    }
}
