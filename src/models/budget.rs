use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{MetaData, NtRef},
    QBCreatable, QBDeletable, QBFullUpdatable,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `Budget`
///
/// A budget allows for an amount to be assigned on a monthly, quarterly, or annual basis for a specific account or customer and are created to give a business measurable expense goals. This amount represents how much should be spent against that account or customer in the give time period.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/budget>
pub struct Budget {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the budget
    pub name: Option<String>,
    /// End date of the budget period
    pub end_date: NaiveDate,
    /// Start date of the budget period
    pub start_date: NaiveDate,
    /// Type of budget entry
    pub budget_entry_type: Option<BudgetEntryType>,
    /// Type of budget
    pub budget_type: Option<BudgetType>,
    /// Details of the budget
    pub budget_detail: Option<Vec<BudgetDetail>>,
    /// Indicates if the budget is active
    pub active: Option<bool>,
    /// Metadata about the entity
    pub meta_data: Option<MetaData>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Default, Serialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct BudgetDetail {
    /// Amount allocated for the budget entry
    pub amount: Option<f64>,
    /// Reference to the account associated with the budget entry
    pub account_ref: Option<NtRef>,
    /// Reference to the class associated with the budget entry
    pub class_ref: Option<NtRef>,
    /// Reference to the customer associated with the budget entry
    pub customer_ref: Option<NtRef>,
    /// Date of the budget entry
    pub budget_date: Option<NaiveDate>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BudgetEntryType {
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BudgetType {
    ProfitAndLoss,
}

impl QBCreatable for Budget {
    fn can_create(&self) -> bool {
        true // start date and end date exist if an instance of budget exists
    }
}

impl QBDeletable for Budget {}
impl QBFullUpdatable for Budget {
    fn can_full_update(&self) -> bool {
        self.id.is_some() && self.sync_token.is_some() && self.can_create()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_budget_deserialization() {
        let input = r#"{
              "StartDate": "2014-01-01",
              "BudgetEntryType": "Monthly",
              "EndDate": "2014-12-31",
              "Name": "Sandbox Budget",
              "SyncToken": "1",
              "BudgetType": "ProfitAndLoss",
              "domain": "QBO",
              "sparse": false,
              "Active": true,
              "BudgetDetail": [
                {
                  "Amount": 0,
                  "AccountRef": {
                    "name": "Services",
                    "value": "1"
                  },
                  "BudgetDate": "2014-01-01"
                },
                {
                  "Amount": 0,
                  "AccountRef": {
                    "name": "Services",
                    "value": "1"
                  },
                  "BudgetDate": "2014-02-01"
                },
                {
                  "Amount": 71.0,
                  "AccountRef": {
                    "name": "Unapplied Cash Payment Income",
                    "value": "87"
                  },
                  "BudgetDate": "2014-12-01"
                }
              ],
              "Id": "1",
              "MetaData": {
                "CreateTime": "2015-07-14T13:59:45-07:00",
                "LastUpdatedTime": "2015-07-14T13:59:59-07:00"
              }
            }"#;
        let budget: Budget = serde_json::from_str(input).unwrap();
        dbg!("Deserialized Budget: {:?}", &budget);
        assert_eq!(budget.id.unwrap(), "1");
        assert_eq!(budget.name.unwrap(), "Sandbox Budget");
    }
}
