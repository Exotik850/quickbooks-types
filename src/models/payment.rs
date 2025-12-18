use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{CreditCardPayment, MetaData, NtRef};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{
    common::TypedRef, Account, CompanyCurrency, Customer, LineField, PaymentMethod, QBCreatable,
    QBDeletable, QBFullUpdatable, QBItem, QBPDFable, QBSendable, QBVoidable,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// `Payment`
///
/// Represents receipt or application of funds against customer balances or invoices.
///
/// Creation requirements:
/// - `QBCreatable::can_create()` returns true when both `total_amt` and `customer_ref` are present.
///
/// Update semantics:
/// - `QBFullUpdatable::can_full_update()` returns true when `has_read()` (ID + sync token) and `can_create()` are both true.
/// - `QBDeletable` and `QBVoidable` are implemented; both require `has_read()`.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/payment>
pub struct Payment {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Total amount of the payment
    pub total_amt: Option<f64>,
    /// Reference to the customer for the payment
    pub customer_ref: Option<TypedRef<Customer>>,
    /// Reference to the currency for the payment
    pub currency_ref: Option<TypedRef<CompanyCurrency>>,
    /// Private note for the payment
    pub private_note: Option<String>,
    /// Reference to the payment method
    pub payment_method_ref: Option<TypedRef<PaymentMethod>>,
    /// Unapplied amount of the payment
    pub unapplied_amt: Option<f64>,
    /// Reference to the account where the payment is deposited
    pub deposit_to_account_ref: Option<TypedRef<Account>>,
    /// Exchange rate for the payment
    pub exchange_rate: Option<f64>,
    /// Line items for the payment
    pub line: Option<LineField>,
    /// Source of the transaction
    pub txn_source: Option<String>,
    /// Reference to the accounts receivable account
    #[serde(rename = "ARAccountRef")]
    pub ar_account_ref: Option<TypedRef<Account>>,
    /// Date of the transaction in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
    /// Information about a credit card payment for the transaction
    pub credit_card_payment: Option<CreditCardPayment>,
    /// Type of location for the transaction
    pub transaction_location_type: Option<String>,
    /// Reference number for the payment
    pub payment_ref_num: Option<String>,
    /// Reference to the tax exemption for the payment
    pub tax_exemption_ref: Option<NtRef>,
}

impl QBCreatable for Payment {
    fn can_create(&self) -> bool {
        self.total_amt.is_some() && self.customer_ref.is_some()
    }
}

impl QBDeletable for Payment {}
impl QBVoidable for Payment {}
impl QBFullUpdatable for Payment {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}

impl QBSendable for Payment {}
impl QBPDFable for Payment {}
