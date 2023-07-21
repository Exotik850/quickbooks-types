use serde::{Deserialize, Serialize};

use super::common::{MetaData, NtRef};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    /// If true, the object is currently enabled for use by QuickBooks.
    pub active: Option<bool>,

    /// Reference to the Inventory Asset account that tracks the current value of the inventory.
    /// If the same account is used for all inventory items, the current balance of this account will represent the current total value of the inventory.
    /// Query the Account name list resource to determine the appropriate Account object for this reference.
    /// Use `Account.id` and `Account.name` from that object for `AssetAccountRef.value` and `AssetAccountRef.name`, respectively.
    ///
    /// Required for Inventory item types.
    pub asset_account_ref: Option<NtRef>,

    /// Description of the item.
    ///
    /// * max character: maximum of 4000 chars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Documentation unavailable.
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,

    /// Reference to the expense account used to pay the vendor for this item.
    /// Must be an account with account type of Cost of Goods Sold.
    /// Query the Account name list resource to determine the appropriate Account object for this reference.
    /// Use `Account.id` and `Account.name` from that object for `ExpenseAccountRef.value` and `ExpenseAccountRef.name`, respectively.
    ///
    /// For France locales:
    /// * This is an optional field.
    /// * This is the purchase account id, If not provided it defaults to the default purchase account: 605100 and 601100 are the default expense accounts used for Service and Product type of item, respectively.
    ///
    /// Required for Inventory, NonInventory, and Service item types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_ref: Option<NtRef>,

    /// Fully qualified name of the entity.
    /// The fully qualified name prepends the topmost parent, followed by each sub element separated by colons.
    /// Takes the form of Item:SubItem.
    /// Returned from an existing object and not input on a new object.
    /// Limited to 5 levels.
    ///
    /// * filterable
    /// * read only
    /// * system defined
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fully_qualified_name: String,

    /// Unique Identifier for an Intuit entity (object).
    ///
    /// Required for the update operation.
    ///
    /// * filterable
    /// * read only
    /// * sortable
    /// * system defined
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,

    #[serde(default)]
    pub income_account_ref: NtRef,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub inv_start_date: String,

    /// Classification that specifies the use of this item.
    /// Available when endpoint is evoked with the minorversion=3 query parameter.
    /// Read-only after object is created.
    /// Valid values include: Product and Service.
    ///
    /// Applicable for France companies only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_category_type: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty", rename = "Type")]
    pub item_type: String,

    #[serde(default)]
    pub level: i64,

    pub meta_data: MetaData,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(default)]
    pub parent_ref: NtRef,

    #[serde(default)]
    pub purchase_cost: f32,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub purchase_desc: String,

    #[serde(default)]
    pub qty_on_hand: i64,

    pub sku: Option<String>,

    #[serde(default, rename = "sparse")]
    pub sparse: bool,

    #[serde(default)]
    pub sub_item: bool,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,

    #[serde(default)]
    pub taxable: bool,

    #[serde(default)]
    pub track_qty_on_hand: bool,

    #[serde(default)]
    pub unit_price: f32,
}
