use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::{Addr, CustomField, Email, LinkedTxn, MetaData, NtRef};
use super::line::Line;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
    #[serde(default)]
    txn_date: NaiveDate,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    print_status: String,
    #[serde(default)]
    sales_term_ref: NtRef,
    #[serde(default)]
    total_amt: f32,
    #[serde(default)]
    line: Vec<Line>,
    #[serde(default)]
    due_date: NaiveDate,
    #[serde(default)]
    sparse: bool,
    #[serde(default)]
    doc_number: String,
    #[serde(default)]
    customer_ref: NtRef,
    #[serde(default)]
    txn_tax_detail: TxnTaxDetail,
    #[serde(default)]
    sync_token: String,
    #[serde(default)]
    linked_txn: Vec<LinkedTxn>,
    #[serde(default)]
    bill_email: Email,
    #[serde(default)]
    ship_addr: Addr,
    #[serde(default)]
    email_status: String,
    #[serde(default)]
    bill_addr: Addr,
    #[serde(default)]
    meta_data: MetaData,
    #[serde(default)]
    custom_field: Vec<CustomField>,
    #[serde(default)]
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct TxnTaxDetail {
    #[serde(default)]
    txn_tax_code_ref: NtRef,
    #[serde(default)]
    total_tax: f32,
    #[serde(default)]
    tax_line: Line,
}
