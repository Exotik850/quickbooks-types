use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef, TxnTaxDetail},
    line::Line,
};

/*
    Invoice Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/invoice
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Invoice {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub meta_data: Option<MetaData>,
    pub line: Vec<Line>,
    pub customer_ref: NtRef,
    pub txn_date: Option<NaiveDate>,
    pub domain: Option<String>,
    pub print_status: Option<String>,
    pub sales_term_ref: Option<NtRef>,
    pub total_amt: Option<f32>,
    pub due_date: Option<NaiveDate>,
    pub sparse: Option<bool>,
    pub doc_number: Option<String>,
    pub txn_tax_detail: Option<TxnTaxDetail>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
    pub bill_email: Option<Email>,
    pub ship_addr: Option<Addr>,
    pub email_status: Option<String>,
    pub bill_addr: Option<Addr>,
    pub custom_field: Option<Vec<CustomField>>,
}
