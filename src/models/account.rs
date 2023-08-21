use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{QBCreatable, QBError, QBFullUpdatable, QBItem, QBToRef};

use super::common::{MetaData, NtRef};

/*
    Account Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(
    feature = "builder",
    builder(default, build_fn(error = "QBError"), setter(into, strip_option))
)]
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
    pub current_balance_with_sub_accounts: Option<f32>,
    pub account_alias: Option<String>,
    pub tax_code_ref: Option<NtRef>,
    pub account_sub_type: Option<String>,
    pub current_balance: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO, // TODO Make this
}

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

impl QBToRef for Account {
    fn ref_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
