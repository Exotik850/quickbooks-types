use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{LinkedTxn, NtRef},
    line::Line,
    qb_object_data::QBObjectData,
};

/*
    Bill Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/bill
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct Bill {
    #[serde(flatten)]
    qb_data: QBObjectData,
    domain: Option<String>,
    #[serde(rename = "APAccountRef")]
    ap_account_ref: Option<NtRef>,
    vendor_ref: NtRef,
    txn_date: Option<NaiveDate>,
    total_amt: Option<f64>,
    currency_ref: Option<NtRef>,
    linked_txn: Option<Vec<LinkedTxn>>,
    sales_term_ref: Option<NtRef>,
    due_date: Option<NaiveDate>,
    sparse: Option<bool>,
    line: Vec<Line>,
    balance: Option<f32>,
    doc_number: Option<String>,
    private_note: Option<String>,
    exchange_rate: Option<f32>,
    department_ref: Option<NtRef>,
    home_balance: Option<f32>,
    recur_data_ref: Option<NtRef>,
}
