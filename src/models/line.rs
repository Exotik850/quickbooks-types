use chrono::{DateTime, NaiveDate, Utc};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{LinkedTxn, NtRef};
use crate::QBCreatable;

pub type LineField = Vec<Line>;

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
/// Line object
///
/// No documentation page, but used as a detail for purchased items or services
pub struct Line {
    /// Details of the line item
    #[serde(flatten)]
    pub line_detail: LineDetail,
    /// Amount total for the line item
    pub amount: Option<f64>,
    /// Description of the line item
    pub description: Option<String>,
    /// Unique line number
    pub id: Option<String>,
    /// Linked transactions
    pub linked_txn: Option<Vec<LinkedTxn>>,
}

impl QBCreatable for Line {
    fn can_create(&self) -> bool {
        self.amount.is_some()
    }
}

impl QBCreatable for Option<LineField> {
    fn can_create(&self) -> bool {
        if let Some(data) = self {
            data.can_create()
        } else {
            false
        }
    }
}

impl QBCreatable for LineField {
    fn can_create(&self) -> bool {
        self.iter().all(QBCreatable::can_create)
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
            LineDetail::None => panic!("Cannot serialize Line Detail of None!"),
        };

        state.serialize_field("DetailType", detail_type)?;
        state.end()
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("Could not serialize Line for display!")
        )
    }
}

/// LineDetail Enum
/// 
/// Subtype of the line detail
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
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
    #[default]
    None,
}

pub trait TaxableLine {
    fn set_taxable(&mut self);
}

impl TaxableLine for LineDetail {
    fn set_taxable(&mut self) {
        if let LineDetail::SalesItemLineDetail(data) = self {
            data.tax_code_ref = Some("TAX".into());
        }
    }
}

impl TaxableLine for Line {
    fn set_taxable(&mut self) {
        self.line_detail.set_taxable();
    }
}

impl TaxableLine for LineField {
    fn set_taxable(&mut self) {
        self.iter_mut().for_each(TaxableLine::set_taxable);
    }
}

impl TaxableLine for Option<LineField> {
    fn set_taxable(&mut self) {
        self.iter_mut().for_each(TaxableLine::set_taxable);
    }
}

impl<'a, T> TaxableLine for std::slice::IterMut<'a, T>
where
    T: TaxableLine,
{
    fn set_taxable(&mut self) {
        self.for_each(TaxableLine::set_taxable);
    }
}

/// SalesItemLineDetail
/// 
/// Description of the sales item line detail
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

pub struct SalesItemLineDetail {
    pub tax_inclusive_amt: Option<f64>,
    pub discount_amt: Option<f64>,
    pub item_ref: Option<NtRef>,
    pub class_ref: Option<NtRef>,
    pub tax_code_ref: Option<NtRef>,
    pub service_date: Option<NaiveDate>,
    pub discount_rate: Option<f64>,
    pub qty: Option<f64>,
    pub unit_price: Option<f64>,
    pub tax_classification_ref: Option<NtRef>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct GroupLineDetail {
    pub quantity: f64,
    pub line: LineField,
    pub group_item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct DescriptionLineDetail {
    pub tax_code_ref: NtRef,
    pub service_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct DiscountLineDetail {
    pub class_ref: NtRef,
    pub tax_code_ref: NtRef,
    pub discount_account_ref: NtRef,
    pub percent_based: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SubTotalLineDetail {
    pub item_ref: NtRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub enum BillableStatus {
    #[default]
    Billable,
    NotBillable,
    HasBeenBilled,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct ItemBasedExpenseLineDetail {
    pub tax_inclusive_amt: f64,
    pub item_ref: NtRef,
    pub customer_ref: NtRef,
    pub price_level_ref: NtRef,
    pub class_ref: NtRef,
    pub tax_code_ref: NtRef,
    pub billable_status: BillableStatus,
    pub qty: f64,
    pub unit_price: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct AccountBasedExpenseLineDetail {
    pub account_ref: NtRef,
    pub tax_code_ref: NtRef,
    pub tax_amount: f64,
    pub tax_inclusive_amt: f64,
    pub class_ref: NtRef,
    pub customer_ref: NtRef,
    pub billable_status: BillableStatus,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TaxLineDetail {
    pub tax_rate_ref: Option<NtRef>,
    pub net_amount_taxable: Option<f64>,
    pub percent_based: Option<bool>,
    pub tax_inclusive_amount: Option<f64>,
    pub override_delta_amount: Option<f64>,
    pub tax_percent: Option<f64>,
}

#[test]
fn deserialize_line() {
    let test: LineField = serde_json::from_str(
        r#"[{
      "Description": "Rock Fountain", 
      "DetailType": "SalesItemLineDetail", 
      "SalesItemLineDetail": {
        "TaxCodeRef": {
          "value": "TAX"
        }, 
        "Qty": 1, 
        "UnitPrice": 275, 
        "ItemRef": {
          "name": "Rock Fountain", 
          "value": "5"
        }
      }, 
      "LineNum": 1, 
      "Amount": 275.0, 
      "Id": "1"
    }, 
    {
      "Description": "Fountain Pump", 
      "DetailType": "SalesItemLineDetail", 
      "SalesItemLineDetail": {
        "TaxCodeRef": {
          "value": "TAX"
        }, 
        "Qty": 1, 
        "UnitPrice": 12.75, 
        "ItemRef": {
          "name": "Pump", 
          "value": "11"
        }
      }, 
      "LineNum": 2, 
      "Amount": 12.75, 
      "Id": "2"
    }, 
    {
      "Description": "Concrete for fountain installation", 
      "DetailType": "SalesItemLineDetail", 
      "SalesItemLineDetail": {
        "TaxCodeRef": {
          "value": "TAX"
        }, 
        "Qty": 5, 
        "UnitPrice": 9.5, 
        "ItemRef": {
          "name": "Concrete", 
          "value": "3"
        }
      }, 
      "LineNum": 3, 
      "Amount": 47.5, 
      "Id": "3"
    }, 
    {
      "DetailType": "SubTotalLineDetail", 
      "Amount": 335.25, 
      "SubTotalLineDetail": {}
    }
  ]"#,
    )
    .unwrap();
    dbg!(test);
}
