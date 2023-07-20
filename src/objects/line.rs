use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::{LinkedTxn, NtRef};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    #[serde(flatten)]
    pub line_detail: LineDetail,
    #[serde(default)]
    pub amount: f32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_txn: Vec<LinkedTxn>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "DetailType")]
pub enum LineDetail {
    // For Invoice type
    SalesItemLine(SalesItemLineDetail),
    GroupLine(GroupLineDetail),
    DescriptionOnlyLine(DescriptionLineDetail),
    DiscountLine(DiscountLineDetail),
    SubTotalLine(SubTotalLineDetail),

    // For Bill type
    ItemBasedExpenseLine(ItemBasedExpenseLineDetail),   
    AccountBasedExpenseLine(AccountBasedExpenseLineDetail),

}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct SalesItemLineDetail {
    tax_inclusive_amt: f32,
    discount_amt: f32,
    item_ref: NtRef,
    class_ref: NtRef,
    tax_code_ref: NtRef,
    service_date: DateTime<Utc>,
    discount_rate: f32,
    qty: u32,
    unit_price: f32,
    tax_classification_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct GroupLineDetail {
    quantity: f32,
    line: Vec<Line>,
    group_item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct DescriptionLineDetail {
    tax_code_ref: NtRef,
    service_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct DiscountLineDetail {
    class_ref: NtRef,
    tax_code_ref: NtRef,
    discount_account_ref: NtRef,
    percent_based: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_percent: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct SubTotalLineDetail {
    item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
enum BillableStatus {
    #[default] Billable,
    NotBillable,
    HasBeenBilled,
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
struct ItemBasedExpenseLineDetail {
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
#[serde(default, rename_all = "PascalCase")]
pub struct AccountBasedExpenseLineDetail {
    account_ref: NtRef,
    tax_code_ref: NtRef,
    tax_amount: f32,
    tax_inclusive_amt: f32,
    class_ref: NtRef,
    customer_ref: NtRef,
    billable_status: BillableStatus,
}
