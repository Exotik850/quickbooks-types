use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{QBCreatable, QBDeletable};

use super::{
    common::{CreditCardPayment, MetaData, NtRef},
    Line,
};

/*
    Payment Object:
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/payment
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option), default))]
pub struct Payment {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub total_amt: Option<f32>,
    pub customer_ref: Option<NtRef>,
    pub currency_ref: Option<NtRef>,
    pub private_note: Option<String>,
    pub payment_method_ref: Option<NtRef>,
    pub unapplied_amt: Option<f32>,
    pub deposit_to_account_ref: Option<NtRef>,
    pub exchange_rate: Option<f32>,
    pub line: Option<Vec<Line>>,
    pub txn_source: Option<String>,
    #[serde(rename = "ARAccountRef")]
    pub ar_account_ref: Option<NtRef>,
    pub txn_date: Option<NaiveDate>,
    pub credit_card_payment: Option<CreditCardPayment>,
    pub transaction_location_type: Option<String>,
    pub payment_ref_num: Option<String>,
    pub tax_exemption_ref: Option<NtRef>,
}

impl QBCreatable for Payment {
    fn can_create(&self) -> bool {
        self.total_amt.is_some() && self.customer_ref.is_some()
    }
}

impl QBDeletable for Payment {}