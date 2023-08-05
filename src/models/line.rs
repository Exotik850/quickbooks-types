use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::QBCreatable;

use super::common::{LinkedTxn, NtRef};

/*
    Line object
    No documentation page, but used as a detail for purchased items or services
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
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

impl QBCreatable for Line {
    fn can_create(&self) -> bool {
        self.line_detail.is_some() && self.amount.is_some()
    }
}

impl QBCreatable for Option<Vec<Line>> {
    fn can_create(&self) -> bool {
        if let Some(data) = self {
            data.can_create()
        } else {
            false
        }
    }
}

impl QBCreatable for Vec<Line> {
    fn can_create(&self) -> bool {
        self.iter().all(|l| l.can_create())
    }
}

impl Serialize for LineDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("LineDetail", 2)?;

        // TODO Make this more generic, although there won't be more types to add in the future most likely
        let detail_type = match self {
            LineDetail::SalesItemLineDetail(data) => {
                state.serialize_field("SalesItemLineDetail", data)?;
                "SalesItemLineDetail"
            }
            LineDetail::GroupLineDetail(data) => {
                state.serialize_field("GroupLineDetail", data)?;
                "GroupLineDetail"
            }
            LineDetail::DescriptionLineDetail(data) => {
                state.serialize_field("DescriptionLineDetail", data)?;
                "DescriptionLineDetail"
            }
            LineDetail::DiscountLineDetail(data) => {
                state.serialize_field("DiscountLineDetail", data)?;
                "DiscountLineDetail"
            }
            LineDetail::SubTotalLineDetail(data) => {
                state.serialize_field("SubTotalLineDetail", data)?;
                "SubTotalLineDetail"
            }
            LineDetail::ItemBasedExpenseLineDetail(data) => {
                state.serialize_field("ItemBasedExpenseLineDetail", data)?;
                "ItemBasedExpenseLineDetail"
            }
            LineDetail::AccountBasedExpenseLineDetail(data) => {
                state.serialize_field("AccountBasedExpenseLineDetail", data)?;
                "AccountBasedExpenseLineDetail"
            }
            LineDetail::TaxLineDetail(data) => {
                state.serialize_field("TaxLineDetail", data)?;
                "TaxLineDetail"
            }
        };

        state.serialize_field("DetailType", detail_type)?;
        state.end()
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
// #[serde(tag = "DetailType")]
pub enum LineDetail {
    SalesItemLineDetail(SalesItemLineDetail),
    GroupLineDetail(GroupLineDetail),
    DescriptionLineDetail(DescriptionLineDetail),
    DiscountLineDetail(DiscountLineDetail),
    SubTotalLineDetail(SubTotalLineDetail),
    ItemBasedExpenseLineDetail(ItemBasedExpenseLineDetail),
    AccountBasedExpenseLineDetail(AccountBasedExpenseLineDetail),
    TaxLineDetail(TaxLineDetail),
}

pub trait TaxableLine {
    fn set_taxable(&mut self);
}

impl TaxableLine for LineDetail {
    fn set_taxable(&mut self) {
        if let LineDetail::SalesItemLineDetail(data) = self {
            data.tax_code_ref.value = Some("TAX".into())
        }
    }
}

impl TaxableLine for Line {
    fn set_taxable(&mut self) {
        if let Some(detail) = self.line_detail.as_mut() {
            detail.set_taxable()
        }
    }
}

impl TaxableLine for Vec<Line> {
    fn set_taxable(&mut self) {
        self.iter_mut().for_each(|f| f.set_taxable())
    }
}

impl TaxableLine for Option<Vec<Line>> {
    fn set_taxable(&mut self) {
        self.iter_mut().for_each(|f| f.set_taxable())
    }
}

impl<'a, T> TaxableLine for std::slice::IterMut<'a, T>
where
    T: TaxableLine,
{
    fn set_taxable(&mut self) {
        self.for_each(|f| f.set_taxable());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
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
#[serde(
    default,
    rename_all = "PascalCase",
    tag = "AccountBasedExpenseLineDetail"
)]
pub struct AccountBasedExpenseLineDetail {
    account_ref: NtRef,
    tax_code_ref: NtRef,
    tax_amount: f32,
    tax_inclusive_amt: f32,
    class_ref: NtRef,
    customer_ref: NtRef,
    billable_status: BillableStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default, tag = "TaxLineDetail")]
pub struct TaxLineDetail {
    tax_rate_ref: NtRef,
    net_amount_taxable: f32,
    percent_based: bool,
    tax_inclusive_amount: f32,
    override_delta_amount: f32,
    tax_percent: f32,
}
