use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef, TxnTaxDetail},
    Line,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Estimate {
    pub id: Option<String>,
    pub sync_token: Option<String>,
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
    pub sales_term_ref: Option<NtRef>,
    pub txn_status: Option<String>,
    pub global_tax_calculation: Option<String>,
    pub accepted_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub doc_number: Option<String>,
    pub private_note: Option<String>,
    pub customer_memo: Option<NtRef>,
    pub email_status: Option<String>,
    pub txn_tax_detail: Option<TxnTaxDetail>,
    pub line: Option<Vec<Line>>,
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
