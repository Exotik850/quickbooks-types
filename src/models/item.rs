use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{MetaData, NtRef};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
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
    pub description: Option<String>,

    /// Documentation unavailable.
    #[serde(rename = "domain")]
    pub domain: Option<String>,

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
    pub fully_qualified_name: Option<String>,

    /// Unique Identifier for an Intuit entity (object).
    ///
    /// Required for the update operation.
    ///
    /// * filterable
    /// * read only
    /// * sortable
    /// * system defined
    pub id: Option<String>,
    pub income_account_ref: Option<NtRef>,
    pub inv_start_date: Option<String>,

    /// Classification that specifies the use of this item.
    /// Available when endpoint is evoked with the minorversion=3 query parameter.
    /// Read-only after object is created.
    /// Valid values include: Product and Service.
    ///
    /// Applicable for France companies only.
    pub item_category_type: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: Option<String>,
    pub level: Option<i64>,
    pub meta_data: MetaData,
    pub name: Option<String>,
    pub parent_ref: Option<NtRef>,
    pub purchase_cost: Option<f32>,
    pub purchase_desc: Option<String>,
    pub qty_on_hand: Option<i64>,
    pub sku: Option<String>,
    #[serde(rename = "sparse")]
    pub sparse: Option<bool>,
    pub sub_item: Option<bool>,
    pub sync_token: Option<String>,
    pub taxable: Option<bool>,
    pub track_qty_on_hand: Option<bool>,
    pub unit_price: Option<f32>,
}
