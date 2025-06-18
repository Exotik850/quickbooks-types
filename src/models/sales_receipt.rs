use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{
        Addr, CreditCardPayment, CustomField, DeliveryInfo, Email, GlobalTaxCalculation, LinkedTxn,
        MetaData, NtRef, PrintStatus, TxnTaxDetail,
    },
    LineField,
};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{QBCreatable, QBFullUpdatable, QBPDFable, QBSendable, QBSparseUpdateable, QBVoidable};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Sales Receipt
///
/// Represents a sales receipt in QuickBooks.
///
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesreceipt>
pub struct SalesReceipt {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// Line items for the transaction
    pub line: Option<LineField>,
    /// Reference to the customer for the transaction
    pub customer_ref: Option<NtRef>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Reference to the currency for the transaction
    pub currency_ref: Option<NtRef>,
    /// Email address for billing
    pub bill_email: Option<Email>,
    /// Address from which the items are shipped
    pub ship_from_addr: Option<Addr>,
    /// Custom fields for the entity
    pub custom_field: Option<Vec<CustomField>>,
    /// Date when the items are shipped
    pub ship_date: Option<NaiveDate>,
    /// Tracking number for the shipment
    pub tracking_num: Option<String>,
    /// Reference to the class for the transaction
    pub class_ref: Option<NtRef>,
    /// Print status of the sales receipt
    pub print_status: Option<PrintStatus>,
    /// Reference number for the payment
    pub payment_ref_num: Option<String>,
    /// Source of the transaction
    pub txn_source: Option<String>,
    /// Linked transactions
    pub linked_txn: Option<Vec<LinkedTxn>>,
    /// Global tax calculation method
    pub global_tax_calculation: Option<GlobalTaxCalculation>,
    /// Indicates if tax is applied after discount
    pub apply_tax_after_discount: Option<bool>,
    /// Document number for the sales receipt
    pub doc_number: Option<String>,
    /// Private note for the transaction
    pub private_note: Option<String>,
    /// Reference to the account where the deposit is made
    pub deposit_to_account_ref: Option<NtRef>,
    /// Memo for the customer
    pub customer_memo: Option<NtRef>,
    /// Information about a credit card payment for the transaction
    pub credit_card_payment: Option<CreditCardPayment>,
    /// Tax details for the transaction
    pub txn_tax_detail: Option<TxnTaxDetail>,
    /// Reference to the payment method for the transaction
    pub payment_method_ref: Option<NtRef>,
    /// Exchange rate for the transaction
    pub exchange_rate: Option<f64>,
    /// Address to which the items are shipped
    pub ship_addr: Option<Addr>,
    /// Indicates if the transaction is a sparse object
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    /// Reference to the department for the transaction
    pub department_ref: Option<NtRef>,
    /// Reference to the shipping method for the transaction
    pub ship_method_ref: Option<NtRef>,
    /// Address for billing
    pub bill_addr: Option<Addr>,
    /// Metadata about the transaction
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Home balance for the transaction
    pub home_balance: Option<f64>,
    /// Delivery information for the transaction
    pub delivery_info: Option<DeliveryInfo>,
    /// Reference to the recurring data for the transaction
    pub recur_data_ref: Option<NtRef>,
    /// Total amount of the transaction
    pub total_amt: Option<f64>,
    /// Balance for the transaction
    pub balance: Option<f64>,
    /// Indicates if the address is a free-form address
    pub free_form_address: Option<bool>,
    /// Date of the transaction in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
}

impl QBCreatable for SalesReceipt {
    fn can_create(&self) -> bool {
        self.line.can_create()
    }
}

impl QBVoidable for SalesReceipt {}
impl QBFullUpdatable for SalesReceipt {
    fn can_full_update(&self) -> bool {
        self.can_create()
    }
}

impl QBSparseUpdateable for SalesReceipt {
    fn can_sparse_update(&self) -> bool {
        self.can_full_update() && self.sparse.is_some_and(|x| x)
    }
}

impl QBSendable for SalesReceipt {}
impl QBPDFable for SalesReceipt {}
