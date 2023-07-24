use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{common::NtRef, qb_object_data::QBObjectData};

/*
    Account Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default, Builder)]
#[serde(rename_all = "PascalCase", default)]
#[builder(setter(into, strip_option), default)]
pub struct Account {
    #[serde(flatten)]
    qb_data: QBObjectData,
    name: Option<String>,
    acct_num: Option<String>,
    currency_ref: Option<NtRef>,
    parent_ref: Option<NtRef>,
    descripton: Option<String>,
    active: Option<bool>,
    sub_account: Option<bool>,
    classification: Option<String>,
    fully_qualified_name: Option<String>,
    txn_location_type: Option<String>,
    account_type: Option<AccountType>,
    current_balance_with_sub_accounts: Option<f32>,
    account_alias: Option<String>,
    tax_code_ref: Option<NtRef>,
    account_sub_type: Option<String>,
    current_balance: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO, // TODO Make this
}
