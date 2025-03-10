use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{MetaData, NtRef};
use crate::{QBCreatable, QBFullUpdatable, QBItem};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// Account Object
///
/// https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account
pub struct Account {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    pub name: Option<String>,
    pub acct_num: Option<String>,
    pub currency_ref: Option<NtRef>,
    pub parent_ref: Option<NtRef>,
    pub descripton: Option<String>,
    pub active: Option<bool>,
    pub sub_account: Option<bool>,
    pub classification: Option<String>,
    pub fully_qualified_name: Option<String>,
    pub txn_location_type: Option<String>,
    pub account_type: Option<AccountType>,
    pub current_balance_with_sub_accounts: Option<f64>,
    pub account_alias: Option<String>,
    pub tax_code_ref: Option<NtRef>,
    pub account_sub_type: Option<String>,
    pub current_balance: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO,
}

// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub enum AccountType {
//     Bank(BankSubType),
//     OtherAsset(OtherAssetSubType),
//     OtherCurrentAsset(OtherCurrentAssetSubType),
//     FixedAsset(FixedAssetSubType)
// }
// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub enum BankSubType {
//     #[default]
//     TODO,
// }
// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub enum OtherAssetSubType {
//     #[default]
//     TODO,
// }
// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub enum FixedAssetSubType {
//     #[default]
//     TODO,
// }
// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub enum BankSubAccountType {
//     #[default]
//     TODO,
// }

impl QBCreatable for Account {
    fn can_create(&self) -> bool {
        self.name.is_some() && (self.account_type.is_some() || self.account_sub_type.is_some())
    }
}

impl QBFullUpdatable for Account {
    fn can_full_update(&self) -> bool {
        self.name.is_some() && self.has_read()
    }
}
