use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{LinkedTxn, NtRef};

/*
    Line object
    No documentation page, but used as a detail for purchased items or services
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(setter(into), default))]
pub struct Line {
    #[serde(flatten)]
    pub line_detail: Option<LineDetail>,
    pub amount: Option<f32>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub linked_txn: Option<Vec<LinkedTxn>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "DetailType")]
pub enum LineDetail {
    SalesItemLineDetail(SalesItemLineDetail),
    GroupLineDetail(GroupLineDetail),
    DescriptionLineDetail(DescriptionLineDetail),
    DiscountLineDetail(DiscountLineDetail),
    SubTotalLineDetail(SubTotalLineDetail),
    ItemBasedExpenseLineDetail(ItemBasedExpenseLineDetail),
    AccountBasedExpenseLineDetail(AccountBasedExpenseLineDetail),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "SalesItemLineDetail")]
pub struct SalesItemLineDetail {
    pub tax_inclusive_amt: f32,
    pub discount_amt: f32,
    pub item_ref: NtRef,
    pub class_ref: NtRef,
    pub tax_code_ref: NtRef,
    pub service_date: DateTime<Utc>,
    pub discount_rate: f32,
    pub qty: u32,
    pub unit_price: f32,
    pub tax_classification_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "GroupLineDetail")]
pub struct GroupLineDetail {
    quantity: f32,
    line: Vec<Line>,
    group_item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "DescriptionLineDetail")]
pub struct DescriptionLineDetail {
    tax_code_ref: NtRef,
    service_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "DiscountLineDetail")]
pub struct DiscountLineDetail {
    class_ref: NtRef,
    tax_code_ref: NtRef,
    discount_account_ref: NtRef,
    percent_based: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_percent: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "SubTotalLineDetail")]
pub struct SubTotalLineDetail {
    item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub enum BillableStatus {
    #[default]
    Billable,
    NotBillable,
    HasBeenBilled,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "ItemBasedExpenseLineDetail")]
pub struct ItemBasedExpenseLineDetail {
    tax_inclusive_amt: f32,
    item_ref: NtRef,
    customer_ref: NtRef,
    price_level_ref: NtRef,
    class_ref: NtRef,
    tax_code_ref: NtRef,
    billable_status: BillableStatus,
    qty: f32,
    unit_price: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase", tag = "AccountBasedExpenseLineDetail")]
pub struct AccountBasedExpenseLineDetail {
    account_ref: NtRef,
    tax_code_ref: NtRef,
    tax_amount: f32,
    tax_inclusive_amt: f32,
    class_ref: NtRef,
    customer_ref: NtRef,
    billable_status: BillableStatus,
}
