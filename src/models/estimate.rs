use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef, TxnTaxDetail};
use crate::{
    common::EmailStatus, LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBPDFable, QBSendable, QBSparseUpdateable
};

#[cfg(feature = "builder")]
use crate::error::QBError;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBError"), setter(into, strip_option))
)]

/// Estimate Object
///
/// https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/estimate
pub struct Estimate {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub customer_ref: Option<NtRef>,
    pub currency_ref: Option<NtRef>,
    pub bill_email: Option<Email>,
    pub txn_date: Option<NaiveDate>,
    pub ship_from_addr: Option<Addr>,
    pub ship_date: Option<NaiveDate>,
    pub class_ref: Option<NtRef>,
    pub custom_field: Option<Vec<CustomField>>,
    pub print_status: Option<String>,
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    pub sales_term_ref: Option<NtRef>,
    pub txn_status: Option<String>,
    pub global_tax_calculation: Option<String>,
    pub accepted_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub doc_number: Option<String>,
    pub private_note: Option<String>,
    pub customer_memo: Option<NtRef>,
    pub email_status: Option<EmailStatus>,
    pub txn_tax_detail: Option<TxnTaxDetail>,
    pub line: Option<LineField>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
    pub accepted_by: Option<String>,
    pub exchange_rate: Option<f32>,
    pub ship_addr: Option<Addr>,
    pub department_ref: Option<NtRef>,
    pub ship_method_ref: Option<NtRef>,
    pub bill_addr: Option<Addr>,
    pub apply_tax_after_discount: Option<bool>,
    pub total_amt: Option<f64>,
    pub recur_data_ref: Option<NtRef>,
    pub tax_exemption_ref: Option<NtRef>,
    pub home_total_amt: Option<f32>,
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
