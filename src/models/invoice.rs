use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, CustomField, Email, LinkedTxn, NtRef, TxnTaxDetail},
    line::Line,
    qb_object_data::QBObjectData,
};

/*
    Invoice Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/invoice
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Invoice {
    #[serde(flatten)]
    qb_data: QBObjectData,
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
