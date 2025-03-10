use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{CreditCardPayment, MetaData, NtRef};
use crate::{
    LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBPDFable, QBSendable, QBVoidable
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

/// Payment Object
///
/// https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/payment
pub struct Payment {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub total_amt: Option<f64>,
    pub customer_ref: Option<NtRef>,
    pub currency_ref: Option<NtRef>,
    pub private_note: Option<String>,
    pub payment_method_ref: Option<NtRef>,
    pub unapplied_amt: Option<f64>,
    pub deposit_to_account_ref: Option<NtRef>,
    pub exchange_rate: Option<f64>,
    pub line: Option<LineField>,
    pub txn_source: Option<String>,
    #[serde(rename = "ARAccountRef")]
    pub ar_account_ref: Option<NtRef>,
    pub txn_date: Option<NaiveDate>,
    pub credit_card_payment: Option<CreditCardPayment>,
    pub transaction_location_type: Option<String>,
    pub payment_ref_num: Option<String>,
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
