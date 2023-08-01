use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Line;
use super::common::{NtRef, Email, Addr, CustomField, PrintStatus, LinkedTxn, GlobalTaxCalculation, CreditCardPayment, TxnTaxDetail, MetaData, DeliveryInfo};

/*
    Sales Receipt Object:
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/salesreceipt
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature="builder", derive(Builder))]
#[cfg_attr(feature="builder", builder(setter(into, strip_option), default))]
pub struct SalesReceipt {
    pub id: Option<String>,
    pub line: Option<Vec<Line>>,
    pub customer_ref: Option<NtRef>,
    pub sync_token: Option<String>,
    pub currency_ref: Option<NtRef>,
    pub bill_email: Option<Email>,
    pub ship_from_addr: Option<Addr>,
    pub custom_field: Option<Vec<CustomField>>,
    pub ship_date: Option<NaiveDate>,
    pub tracking_num: Option<String>,
    pub class_ref: Option<NtRef>,
    pub print_status: Option<PrintStatus>,
    pub payment_ref_num: Option<String>,
    pub txn_source: Option<String>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
    pub global_tax_calculation: Option<GlobalTaxCalculation>,
    pub apply_tax_after_discount: Option<bool>,
    pub doc_number: Option<String>,
    pub private_note: Option<String>,
    pub deposit_to_account_ref: Option<NtRef>,
    pub customer_memo: Option<NtRef>,
    pub credit_card_payment: Option<CreditCardPayment>,
    pub txn_tax_detail: Option<TxnTaxDetail>,
    pub payment_method_ref: Option<NtRef>,
    pub exchange_rate: Option<f32>,
    pub ship_addr: Option<Addr>,
    pub department_ref: Option<NtRef>,
    pub ship_method_ref: Option<NtRef>,
    pub bill_addr: Option<Addr>,
    pub meta_data: Option<MetaData>,
    pub home_balance: Option<f32>,
    pub delivery_info: Option<DeliveryInfo>,
    pub recur_data_ref: Option<NtRef>,
    pub total_amt: Option<f64>,
    pub balance: Option<f32>,
    pub free_form_address: Option<bool>,
}