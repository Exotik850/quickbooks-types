use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{Addr, CustomField, Email, LinkedTxn, NtRef, TxnTaxDetail},
    qb_object_data::QBObjectData,
    Line,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct Estimate {
    #[serde(flatten)]
    qb_data: QBObjectData,
    customer_ref: Option<NtRef>,
    currency_ref: Option<NtRef>,
    bill_email: Option<Email>,
    txn_date: Option<NaiveDate>,
    ship_from_addr: Option<Addr>,
    ship_date: Option<NaiveDate>,
    class_ref: Option<NtRef>,
    custom_field: Option<Vec<CustomField>>,
    print_status: Option<String>,
    sales_term_ref: Option<NtRef>,
    txn_status: Option<String>,
    global_tax_calculation: Option<String>,
    accepted_date: Option<NaiveDate>,
    expiration_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    doc_number: Option<String>,
    private_note: Option<String>,
    customer_memo: Option<NtRef>,
    email_status: Option<String>,
    txn_tax_detail: Option<TxnTaxDetail>,
    line: Option<Vec<Line>>,
    linked_txn: Option<Vec<LinkedTxn>>,
}
