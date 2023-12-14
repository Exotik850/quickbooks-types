use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{LinkedTxn, MetaData, NtRef};
use crate::{QBCreatable, QBDeletable, QBError, QBFullUpdatable, QBItem, LineField};

/*
    Bill Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/bill
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBError"), setter(into, strip_option))
)]

pub struct Bill {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub domain: Option<String>,
    #[serde(rename = "APAccountRef")]
    pub ap_account_ref: Option<NtRef>,
    pub vendor_ref: Option<NtRef>,
    pub txn_date: Option<NaiveDate>,
    pub total_amt: Option<f64>,
    pub currency_ref: Option<NtRef>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
    pub sales_term_ref: Option<NtRef>,
    pub due_date: Option<NaiveDate>,
    pub sparse: Option<bool>,
    pub line: Option<LineField>,
    pub balance: Option<f32>,
    pub doc_number: Option<String>,
    pub private_note: Option<String>,
    pub exchange_rate: Option<f32>,
    pub department_ref: Option<NtRef>,
    pub home_balance: Option<f32>,
    pub recur_data_ref: Option<NtRef>,
}

impl QBCreatable for Bill {
    fn can_create(&self) -> bool {
        self.vendor_ref.is_some() && self.line.is_some()
    }
}

impl QBDeletable for Bill {}
impl QBFullUpdatable for Bill {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.vendor_ref.is_some() && self.line.is_some()
    }
}
