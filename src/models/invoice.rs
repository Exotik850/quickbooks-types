use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{
    Addr, CustomField, DeliveryInfo, Email, EmailStatus, LinkedTxn, MetaData, NtRef, PrintStatus,
    TxnTaxDetail,
};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{
    LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBPDFable, QBSendable,
    QBSparseUpdateable, QBVoidable,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Invoice
///
/// Represents a sales transaction billed to a customer creating an accounts receivable balance; consists of line items, taxes, payment terms, and delivery information.
///
/// Update semantics:
/// - `QBCreatable::can_create()` returns true when both `customer_ref` and at least one valid line are present.
/// - `QBFullUpdatable::can_full_update()` requires `has_read()` (ID + sync token) and `can_create()`.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/invoice>
pub struct Invoice {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Shipping address for the invoice
    pub ship_from_addr: Option<Addr>,
    /// Date when the items are shipped
    pub ship_date: Option<NaiveDate>,
    /// Shipping tracking number
    pub tracking_num: Option<String>,
    /// Reference to the class for the invoice
    pub class_ref: Option<NtRef>,
    /// Source of the transaction
    pub txn_source: Option<String>,
    /// Reference to the account where the deposit is made
    pub deposit_to_account_ref: Option<NtRef>,
    /// Indicates if online ACH payment is allowed
    #[serde(rename = "AllowOnlineACHPayment")]
    pub allow_online_ach_payment: Option<bool>,
    /// Line items for the invoice
    pub line: Option<LineField>,
    /// Private note for the invoice
    pub private_note: Option<String>,
    /// Delivery information for the invoice
    pub delivery_info: Option<DeliveryInfo>,
    /// Carbon copy email address for billing emails
    pub bill_email_cc: Option<Email>,
    /// Blind carbon copy email address for billing emails
    pub bill_email_bcc: Option<Email>,
    /// Reference to the shipping method used
    pub ship_method_reef: Option<NtRef>,
    /// Indicates if tax is applied after discount
    pub apply_tax_after_discount: Option<bool>,
    /// Customer memo for the invoice
    pub customer_memo: Option<NtRef>,
    /// Reference to the customer for the invoice
    pub customer_ref: Option<NtRef>,
    /// Date of the transaction in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
    /// Domain of the transaction. `QBO` for `QuickBooks` Online.
    pub domain: Option<String>,
    /// Print status of the invoice
    pub print_status: Option<PrintStatus>,
    /// Reference to the sales terms for the invoice
    pub sales_term_ref: Option<NtRef>,
    /// Exchange rate for the transaction
    pub exchange_rate: Option<f64>,
    /// Deposit amount for the invoice
    pub deposit: Option<f64>,
    /// Indicates if online credit card payment is allowed
    pub allow_online_credit_card_payment: Option<bool>,
    /// Reference to the department for the invoice
    pub department_ref: Option<NtRef>,
    /// Email status of the invoice
    pub email_status: Option<EmailStatus>,
    /// Due date for the invoice
    pub due_date: Option<NaiveDate>,
    /// Balance amount in home currency
    pub home_balance: Option<f64>,
    /// Total amount of the invoice
    pub total_amt: Option<f64>,
    /// URL to the invoice in `QuickBooks` Online
    pub invoice_link: Option<String>,
    /// Reference to recurring template data
    pub recur_data_ref: Option<NtRef>,
    /// Reference to tax exemption information
    pub tax_exemption_ref: Option<NtRef>,
    /// Current balance of the invoice
    pub balance: Option<f64>,
    /// Total amount in home currency
    pub home_total_amt: Option<f64>,
    /// Indicates if the address is free-form
    pub free_form_address: Option<bool>,
    /// Indicates if the entity is a sparse object
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    /// Document number for the invoice
    pub doc_number: Option<String>,
    /// Tax details for the transaction
    pub txn_tax_detail: Option<TxnTaxDetail>,
    /// Linked transactions to this invoice
    pub linked_txn: Option<Vec<LinkedTxn>>,
    /// Email address for billing
    pub bill_email: Option<Email>,
    /// Shipping address for the invoice
    pub ship_addr: Option<Addr>,
    /// Billing address for the invoice
    pub bill_addr: Option<Addr>,
    /// Custom fields for the invoice
    pub custom_field: Option<Vec<CustomField>>,
}

impl QBCreatable for Invoice {
    fn can_create(&self) -> bool {
        self.customer_ref.is_some() && self.line.can_create()
    }
}

impl QBDeletable for Invoice {}
impl QBVoidable for Invoice {}

impl QBFullUpdatable for Invoice {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}

impl QBSparseUpdateable for Invoice {
    fn can_sparse_update(&self) -> bool {
        self.can_full_update()
    }
}

impl QBSendable for Invoice {}
impl QBPDFable for Invoice {}
