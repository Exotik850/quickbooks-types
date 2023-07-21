use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{MetaData, NtRef};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Account {
    id: Option<String>,
    name: Option<String>,
    sync_token: Option<String>,
    acct_num: Option<String>,
    currency_ref: Option<NtRef>,
    parent_ref: Option<NtRef>,
    descripton: Option<String>,
    active: Option<bool>,
    meta_data: Option<MetaData>,
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
