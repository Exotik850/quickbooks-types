use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{MetaData, NtRef},
    LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBVoidable,
};

#[cfg(feature = "builder")]
use crate::error::QBTypeError;

/*
    Bill Payment Object

*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// Bill Payment
///
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/billpayment>
pub struct BillPayment {
    /// The unique ID of the entity
    pub sync_token: Option<String>,
    /// Domain of the transaction. `QBO` for QuickBooks Online.
    pub domain: Option<String>,
    /// Reference to the vendor for the transaction.
    pub vendor_ref: Option<NtRef>,
    /// Date of the transaction in YYYY-MM-DD format.
    pub txn_date: Option<String>,
    /// Total amount of the transaction.
    pub total_amt: Option<f64>,
    /// Type of payment for the transaction.
    pub pay_type: Option<PayType>,
    /// Private note for the transaction
    pub private_note: Option<String>,
    /// Indicates if the transaction is a sparse object
    pub sparse: Option<bool>,
    /// Line items for the transaction
    pub line: Option<LineField>,
    /// The unique ID of the entity
    pub id: Option<String>,
    /// Information about a check payment for the transaction. Not applicable to Estimate and SalesOrder. Used when PayType is `Check`
    pub check_payment: Option<CheckBillPayment>,
    /// Information about a credit card payment for the transaction. Not applicable to Estimate and SalesOrder. Used when PayType is `CreditCard`
    pub credit_card_payment: Option<CreditCardBillPayment>,
    /// Metadata about the transaction
    pub meta_data: Option<MetaData>,
}

/// CheckBillPayment
///
/// Information about a check payment for the transaction.
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckBillPayment {
    pub print_status: Option<String>,
    pub bank_account_ref: Option<NtRef>,
}

/// CreditCardBillPayment
///
/// Information about a credit card payment for the transaction.
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditCardBillPayment {
    #[serde(rename = "CCAccountRef")]
    pub cc_account_ref: Option<NtRef>,
}

/// `PayType` Enum
///
/// Type of payment for the transaction.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PayType {
    #[default]
    CreditCard,
    Check,
}

impl QBCreatable for BillPayment {
    fn can_create(&self) -> bool {
        self.vendor_ref.is_some()
            && self.total_amt.is_some()
            && self.line.is_some()
            && self.pay_type.as_ref().is_some_and(|e| match e {
                PayType::CreditCard => self.credit_card_payment.is_some(),
                PayType::Check => self.check_payment.is_some(),
            })
        // TODO Currency ref check
    }
}

impl QBVoidable for BillPayment {}
impl QBDeletable for BillPayment {}
impl QBFullUpdatable for BillPayment {
    fn can_full_update(&self) -> bool {
        self.can_create() && self.has_read()
    }
}
