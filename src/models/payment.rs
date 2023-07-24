use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    common::{MetaData, NtRef},
    Line,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct Payment {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub meta_data: Option<MetaData>,
    pub total_amt: f32,
    pub customer_ref: NtRef,
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

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct CreditCardPayment {
    credit_charge_response: Option<CreditChargeResponse>,
    credit_charge_info: Option<CreditChargeInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct CreditChargeResponse {
    status: Option<CCPaymentStatus>,
    auth_code: Option<String>,
    txn_authorization_time: Option<DateTime<Utc>>,
    #[serde(rename = "CCTransId")]
    cc_trans_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum CCPaymentStatus {
    Completed,
    #[default]
    Unkown,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct CreditChargeInfo {
    cc_expiry_month: Option<u32>,
    process_payment: Option<bool>,
    postal_code: Option<String>,
    amount: Option<f32>,
    name_on_acct: Option<String>,
    cc_expiry_year: Option<u32>,
    #[serde(rename = "type")]
    card_type: Option<String>,
    bill_addr_street: Option<String>,
}
