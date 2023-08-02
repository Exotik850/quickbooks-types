use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{QBCreatable, QBToRef, QBSparseUpdateable};

use super::{
    common::{
        Addr, CustomField, DeliveryInfo, Email, EmailStatus, LinkedTxn, MetaData, NtRef,
        PrintStatus, TxnTaxDetail,
    },
    line::Line,
};

/*
    Invoice Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/most-commonly-used/invoice
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into), default))]
pub struct Invoice {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub ship_from_addr: Option<Addr>,
    pub ship_date: Option<NaiveDate>,
    pub tracking_num: Option<String>,
    pub class_ref: Option<NtRef>,
    pub txn_source: Option<String>,
    pub deposit_to_account_ref: Option<NtRef>,
    #[serde(rename = "AllowOnlineACHPayment")]
    pub allow_online_ach_payment: Option<bool>,
    pub line: Option<Vec<Line>>,
    pub private_note: Option<String>,
    pub delivery_info: Option<DeliveryInfo>,
    pub bill_email_cc: Option<Email>,
    pub bill_email_bcc: Option<Email>,
    pub ship_method_reef: Option<NtRef>,
    pub apply_tax_after_discount: Option<bool>,
    pub customer_memo: Option<NtRef>,
    pub customer_ref: Option<NtRef>,
    pub txn_date: Option<NaiveDate>,
    pub domain: Option<String>,
    pub print_status: Option<PrintStatus>,
    pub sales_term_ref: Option<NtRef>,
    pub exchange_rate: Option<f32>,
    pub deposit: Option<f32>,
    pub allow_online_credit_card_payment: Option<bool>,
    pub department_ref: Option<NtRef>,
    pub email_status: Option<EmailStatus>,
    pub due_date: Option<NaiveDate>,
    pub home_balance: Option<f32>,
    pub total_amt: Option<f64>,
    pub invoice_link: Option<String>,
    pub recur_data_ref: Option<NtRef>,
    pub tax_exemption_ref: Option<NtRef>,
    pub balance: Option<f32>,
    pub home_total_amt: Option<f32>,
    pub free_form_address: Option<bool>,
    pub sparse: Option<bool>,
    pub doc_number: Option<String>,
    pub txn_tax_detail: Option<TxnTaxDetail>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
    pub bill_email: Option<Email>,
    pub ship_addr: Option<Addr>,
    pub bill_addr: Option<Addr>,
    pub custom_field: Option<Vec<CustomField>>,
}

impl QBCreatable for Invoice {
    fn can_create(&self) -> bool {
        if let Some(data) = &self.line {
            self.customer_ref.is_some() && data.iter().all(|c| c.can_create())
        } else {
            false
        }
    }
}

impl QBToRef for Invoice {
    fn ref_name(&self) -> Option<&String> {
        self.doc_number.as_ref()
    }
}

impl QBSparseUpdateable for Invoice {
    fn can_sparse_update(&self) -> bool {
        self.id.is_some() 
        && self.line.is_some() 
        && self.customer_ref.is_some() 
        && self.sync_token.is_some()
        // TODO add the docnumber check, it's more complicated though
    }
}