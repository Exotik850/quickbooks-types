use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{MetaData, NtRef};
#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{QBCreatable, QBFullUpdatable, QBItem};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// Account
///
/// Represents a general ledger account in QuickBooks Online (for example: Bank, Income, Expense).
/// Accounts categorize transactions and track balances. Many entities reference an account via `*Ref` fields.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/account>
pub struct Account {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// Name of the account
    pub name: Option<String>,
    /// Account number
    pub acct_num: Option<String>,
    /// Reference to the currency for the account
    pub currency_ref: Option<NtRef>,
    /// Reference to the parent account if this is a sub-account
    pub parent_ref: Option<NtRef>,
    /// Description of the account
    pub descripton: Option<String>,
    /// Indicates if the account is active
    pub active: Option<bool>,
    /// Indicates if the account is a sub-account
    pub sub_account: Option<bool>,
    /// Classification of the account
    pub classification: Option<String>,
    /// Fully qualified name of the account
    pub fully_qualified_name: Option<String>,
    /// Location type for transactions in this account
    pub txn_location_type: Option<String>,
    /// Type of account
    pub account_type: Option<AccountType>,
    /// Current balance including sub-accounts
    pub current_balance_with_sub_accounts: Option<f64>,
    /// Alternative name for the account
    pub account_alias: Option<String>,
    /// Reference to the tax code associated with the account
    pub tax_code_ref: Option<NtRef>,
    /// Sub-type of the account
    pub account_sub_type: Option<String>,
    /// Current balance of the account
    pub current_balance: Option<f64>,
}

/// AccountType
///
/// High-level classification of an account (for example: Bank, OtherAsset, Income).
/// Note: This enum is currently a placeholder; concrete variants will be added as the API surface is implemented.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    #[default]
    TODO, // TODO: Define actual types
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
