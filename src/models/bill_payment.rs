use crate::{
    common::{MetaData, NtRef},
    Line, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBVoidable, QBError
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/*
    Bill Payment Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/billpayment
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(default, build_fn(error = "QBError"), setter(into, strip_option)))]
pub struct BillPayment {
    pub sync_token: Option<String>,
    pub domain: Option<String>,
    pub vendor_ref: Option<NtRef>,
    pub txn_date: Option<String>,
    pub total_amt: Option<f64>,
    pub pay_type: Option<PayType>,
    pub private_note: Option<String>,
    pub sparse: Option<bool>,
    pub line: Option<Vec<Line>>,
    pub id: Option<String>,
    pub check_payment: Option<CheckBillPayment>,
    pub credit_card_payment: Option<CreditCardBillPayment>,
    pub meta_data: Option<MetaData>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckBillPayment {
    pub print_status: Option<String>,
    pub bank_account_ref: Option<NtRef>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditCardBillPayment {
    #[serde(rename = "CCAccountRef")]
    pub cc_account_ref: Option<NtRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PayType {
    #[default]
    CreditCard,
    Check,
}

impl QBCreatable for BillPayment {
    fn can_create(&self) -> bool {
        self.vendor_ref.is_some()
            && self.total_amt.is_some()
            && self.line.is_some()
            && self.pay_type.as_ref().is_some_and(|e| match e {
                PayType::CreditCard => self.credit_card_payment.is_some(),
                PayType::Check => self.check_payment.is_some(),
            })
        // TODO Currency ref check
    }
}

impl QBVoidable for BillPayment {}
impl QBDeletable for BillPayment {}
impl QBFullUpdatable for BillPayment {
    fn can_full_update(&self) -> bool {
        self.can_create() && self.has_read()
    }
}
