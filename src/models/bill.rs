use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{LinkedTxn, MetaData};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{
    common::TypedRef, impl_linked, Account, CompanyCurrency, Department, LineField, QBCreatable,
    QBDeletable, QBFullUpdatable, QBItem, RecurringTransaction, Term, Vendor,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// Bill
///
/// Represents an accounts payable bill received from a vendor.
/// Records amounts owed to vendors; line items specify products/services and their costs.
///
/// Creation requirements:
/// - `QBCreatable::can_create()` returns true when both `vendor_ref` and at least one valid `line` are present.
///
/// Update semantics:
/// - `QBFullUpdatable::can_full_update()` returns true when `has_read()` (ID + sync token) is true and the creation requirements are satisfied (`vendor_ref` and `line`).
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/bill>
pub struct Bill {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Domain of the transaction. `QBO` for `QuickBooks` Online.
    pub domain: Option<String>,
    /// Reference to the Accounts Payable account for the transaction
    #[serde(rename = "APAccountRef")]
    pub ap_account_ref: Option<TypedRef<Account>>,
    /// Reference to the vendor for the transaction
    pub vendor_ref: Option<TypedRef<Vendor>>,
    /// Date of the transaction in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
    /// Total amount of the transaction
    pub total_amt: Option<f64>,
    /// Reference to the currency for the transaction
    pub currency_ref: Option<TypedRef<CompanyCurrency>>,
    /// List of transactions linked to this bill
    pub linked_txn: Option<Vec<LinkedTxn>>,
    /// Reference to the sales terms for the transaction
    pub sales_term_ref: Option<TypedRef<Term>>,
    /// Due date of the bill in YYYY-MM-DD format
    pub due_date: Option<NaiveDate>,
    /// Indicates if the transaction is a sparse object
    pub sparse: Option<bool>,
    /// Line items for the transaction
    pub line: Option<LineField>,
    /// Remaining balance on the bill
    pub balance: Option<f64>,
    /// Document number for the bill
    pub doc_number: Option<String>,
    /// Private note for the transaction
    pub private_note: Option<String>,
    /// Exchange rate for the transaction
    pub exchange_rate: Option<f64>,
    /// Reference to the department for the transaction
    pub department_ref: Option<TypedRef<Department>>,
    /// Home currency balance for the transaction
    pub home_balance: Option<f64>,
    /// Reference to recurring schedule information
    pub recur_data_ref: Option<TypedRef<RecurringTransaction>>,
}

impl_linked!(Bill as ap_account_ref => Account);
impl_linked!(Bill as vendor_ref => Vendor);
impl_linked!(Bill as currency_ref => CompanyCurrency);
impl_linked!(Bill as sales_term_ref => Term);
impl_linked!(Bill as department_ref => Department);
impl_linked!(Bill as recur_data_ref => RecurringTransaction);

impl QBCreatable for Bill {
    fn can_create(&self) -> bool {
        self.vendor_ref.is_some() && self.line.is_some()
    }
}

impl QBDeletable for Bill {}
impl QBFullUpdatable for Bill {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.vendor_ref.is_some() && self.line.is_some()
    }
}
