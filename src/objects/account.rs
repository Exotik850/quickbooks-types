use serde::{Deserialize, Serialize};

use super::common::{MetaData, NtRef};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Account {
    id: String,
    name: String,
    sync_token: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    acct_num: String,
    currency_ref: NtRef,
    parent_ref: NtRef,
    #[serde(skip_serializing_if = "String::is_empty")]
    descripton: String,
    active: bool,
    meta_data: MetaData,
    sub_account: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    classification: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    fully_qualified_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    txn_location_type: String,
    account_type: AccountType,
    current_balance_with_sub_accounts: f32,
    #[serde(skip_serializing_if = "String::is_empty")]
    account_alias: String,
    tax_code_ref: NtRef,
    #[serde(skip_serializing_if = "String::is_empty")]
    account_sub_type: String,
    current_balance: f32,
}
