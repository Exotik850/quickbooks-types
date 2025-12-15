use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::{MetaData, NtRef};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
pub struct RecurringTransaction {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the recurring transaction
    pub name: Option<String>,
    pub recur_data_ref: Option<NtRef>,
    pub meta_data: Option<MetaData>,
    #[serde(rename = "Type")]
    pub entity_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct RecurringInfo {
    pub recur_type: Option<RecurType>,
    pub schedule_info: Option<RecurringScheduleInfo>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum RecurType {
    Automated,
    Reminded,
    UnScheduled,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct RecurringScheduleInfo {
    pub day_of_week: Option<Weekday>,
    pub start_date: Option<NaiveDate>,
    pub max_occurrences: Option<u32>,
    pub remind_days: Option<u32>,
    pub interval_type: Option<IntervalType>,
    pub week_of_month: Option<u32>,
    pub month_of_year: Option<String>,
    pub days_before: Option<String>,
    pub next_date: Option<NaiveDate>,
    pub num_interval: Option<u32>,
    pub end_date: Option<NaiveDate>,
    pub previous_date: Option<NaiveDate>,
    pub day_of_month: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum IntervalType {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
