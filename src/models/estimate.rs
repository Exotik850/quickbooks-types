use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef, TxnTaxDetail};
use crate::{
    common::EmailStatus, LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBPDFable,
    QBSendable, QBSparseUpdateable,
};

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
/// Estimate
///
/// An estimate represents a proposal for a financial transaction between a business and its customer.
/// It outlines proposed goods or services and their costs, which may later become an invoice.
///
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/estimate>
pub struct Estimate {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Reference to the customer for the estimate
    pub customer_ref: Option<NtRef>,
    /// Reference to the currency for the estimate
    pub currency_ref: Option<NtRef>,
    /// Email address where the estimate should be sent
    pub bill_email: Option<Email>,
    /// Date of the estimate in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
    /// Address information for where items are shipped from
    pub ship_from_addr: Option<Addr>,
    /// Date the items are expected to ship
    pub ship_date: Option<NaiveDate>,
    /// Reference to the class for the estimate
    pub class_ref: Option<NtRef>,
    /// Custom fields for the estimate
    pub custom_field: Option<Vec<CustomField>>,
    /// Status indicating whether the estimate has been printed
    pub print_status: Option<String>,
    /// Indicates if the entity is a sparse object
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    /// Reference to the sales terms for the estimate
    pub sales_term_ref: Option<NtRef>,
    /// Status of the transaction (e.g., "Pending", "Accepted")
    pub txn_status: Option<String>,
    /// Global tax calculation method
    pub global_tax_calculation: Option<String>,
    /// Date when the estimate was accepted by the customer
    pub accepted_date: Option<NaiveDate>,
    /// Date when the estimate expires
    pub expiration_date: Option<NaiveDate>,
    /// Due date for the estimate
    pub due_date: Option<NaiveDate>,
    /// Document number for the estimate
    pub doc_number: Option<String>,
    /// Private note for the estimate (not visible to customers)
    pub private_note: Option<String>,
    /// Customer memo for the estimate
    pub customer_memo: Option<NtRef>,
    /// Status indicating whether the estimate has been emailed
    pub email_status: Option<EmailStatus>,
    /// Tax details for the transaction
    pub txn_tax_detail: Option<TxnTaxDetail>,
    /// Line items for the estimate
    pub line: Option<LineField>,
    /// Information about transactions linked to this estimate
    pub linked_txn: Option<Vec<LinkedTxn>>,
    /// Name of the person who accepted the estimate
    pub accepted_by: Option<String>,
    /// Exchange rate for the currency
    pub exchange_rate: Option<f64>,
    /// Shipping address for the estimate
    pub ship_addr: Option<Addr>,
    /// Reference to the department for the estimate
    pub department_ref: Option<NtRef>,
    /// Reference to the shipping method for the estimate
    pub ship_method_ref: Option<NtRef>,
    /// Billing address for the estimate
    pub bill_addr: Option<Addr>,
    /// Indicates if tax should be applied after discount
    pub apply_tax_after_discount: Option<bool>,
    /// Total amount of the estimate
    pub total_amt: Option<f64>,
    /// Reference to recurring data for the estimate
    pub recur_data_ref: Option<NtRef>,
    /// Reference to tax exemption information
    pub tax_exemption_ref: Option<NtRef>,
    /// Total amount in home currency
    pub home_total_amt: Option<f64>,
    /// Indicates if the address is free-form (not structured)
    pub free_form_address: Option<bool>,
}

impl QBCreatable for Estimate {
    fn can_create(&self) -> bool {
        self.line.is_some() && self.customer_ref.is_some()
    }
}

impl QBDeletable for Estimate {}

impl QBFullUpdatable for Estimate {
    fn can_full_update(&self) -> bool {
        if !self.has_read() || self.customer_ref.is_none() {
            false
        } else if let Some(EmailStatus::NeedToSend) = self.email_status.as_ref() {
            self.bill_email.is_some()
        } else {
            true
        }
    }
}

impl QBSparseUpdateable for Estimate {
    fn can_sparse_update(&self) -> bool {
        self.can_full_update() && self.sparse.is_some_and(|x| x)
    }
}

impl QBSendable for Estimate {}
impl QBPDFable for Estimate {}
