use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use super::{common::{NtRef, LinkedTxn, MetaData}, line::Line};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="PascalCase", default)]
pub struct Bill {
    sync_token: String,
    domain: String,
    id: String,
    #[serde(rename="APAcountRef")]
    ap_account_ref: NtRef,
    vendor_ref: NtRef,
    txn_date: NaiveDate,
    total_amt: f64,
    currency_ref: NtRef,
    linked_txn: Vec<LinkedTxn>,
    sales_term_ref: NtRef,
    due_date: NaiveDate,
    sparse: bool,
    line: Vec<Line>,
    balance: f32,
    meta_data: MetaData,
    doc_number: String,
    private_note: String,
    exchange_rate: f32,
    department_ref: NtRef,
    home_balance: f32, 
    recur_data_ref: NtRef,
}
