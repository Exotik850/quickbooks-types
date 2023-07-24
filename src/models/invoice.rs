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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct Invoice {
    id: Option<String>,
    sync_token: Option<String>,
    meta_data: Option<MetaData>,
    line: Vec<Line>,
    customer_ref: NtRef,
    txn_date: Option<NaiveDate>,
    domain: Option<String>,
    print_status: Option<String>,
    sales_term_ref: Option<NtRef>,
    total_amt: Option<f32>,
    due_date: Option<NaiveDate>,
    sparse: Option<bool>,
    doc_number: Option<String>,
    txn_tax_detail: Option<TxnTaxDetail>,
    linked_txn: Option<Vec<LinkedTxn>>,
    bill_email: Option<Email>,
    ship_addr: Option<Addr>,
    email_status: Option<String>,
    bill_addr: Option<Addr>,
    custom_field: Option<Vec<CustomField>>,
}
