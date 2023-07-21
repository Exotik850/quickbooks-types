use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef};
use super::line::Line;

/*
    Invoice Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/invoice
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Invoice {
    id: Option<String>,
    line: Vec<Line>,
    customer_ref: NtRef,
    sync_token: Option<String>,
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
    meta_data: Option<MetaData>,
    custom_field: Option<Vec<CustomField>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct TxnTaxDetail {
    txn_tax_code_ref: Option<NtRef>,
    total_tax: Option<f32>,
    tax_line: Option<Vec<Line>>,
}
