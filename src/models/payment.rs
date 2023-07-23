use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{common::NtRef, qb_object_data::QBObjectData, Line};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct Payment {
    #[serde(flatten)]
    qb_data: QBObjectData,
    total_amt: f32,
    customer_ref: NtRef,
    currency_ref: Option<NtRef>,
    private_note: Option<String>,
    payment_method_ref: Option<NtRef>,
    unapplied_amt: Option<f32>,
    deposit_to_account_ref: Option<NtRef>,
    exchange_rate: Option<f32>,
    line: Option<Vec<Line>>,
    txn_source: Option<String>,
    #[serde(rename = "ARAccountRef")]
    ar_account_ref: Option<NtRef>,
    txn_date: Option<NaiveDate>,
    credit_card_payment: Option<CreditCardPayment>,
    transaction_location_type: Option<String>,
    payment_ref_num: Option<String>,
    tax_exemption_ref: Option<NtRef>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct CreditCardPayment {
    credit_charge_response: Option<CreditChargeResponse>,
    credit_charge_info: Option<CreditChargeInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct CreditChargeResponse {
    status: Option<CCPaymentStatus>,
    auth_code: Option<String>,
    txn_authorization_time: Option<DateTime<Utc>>,
    #[serde(rename = "CCTransId")]
    cc_trans_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
enum CCPaymentStatus {
    Completed,
    #[default]
    Unkown,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct CreditChargeInfo {
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
