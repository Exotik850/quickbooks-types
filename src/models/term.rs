use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::MetaData, QBCreatable, QBFullUpdatable, QBItem};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Term
///
/// represents the terms under which a sale is made, typically expressed in the form of days due after the goods are received.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/term>
pub struct Term {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the term
    pub name: Option<String>,
    /// Percentage discount offered if paid within "`DiscountDays`"
    pub discount_percent: Option<u16>,
    /// Number of days within which a payment must be made to avail the discount
    ///
    /// used only when "`due_days`" is set
    pub discount_days: Option<u16>,
    /// Indicates if the term is active
    pub active: Option<bool>,
    /// Type of the term
    #[serde(rename = "type")]
    pub term_type: Option<TermType>,
    /// Metadata about the entity
    pub meta_data: Option<MetaData>,
    /// Day of the month when the payment is due
    ///
    /// required if "`due_days`" is not set
    pub day_of_month_due: Option<u16>,
    /// Day of the month when the discount is applicable
    ///
    /// required if "`due_days`" is not set
    pub discount_day_of_month: Option<u16>,
    /// Payment due next month if issued that many days before the `DayOfMonthDue`. Required if `DueDays` not present.
    pub due_next_month_days: Option<u16>,
    /// Number of days from delivery of goods or services until the payment is due. Required if `DayOfMonthDue` not present
    pub due_days: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TermType {
    Standard,
    DateDriven,
}

impl QBCreatable for Term {
    fn can_create(&self) -> bool {
        self.name.is_some() && (self.day_of_month_due.is_some() || self.due_days.is_some())
    }
}

impl QBFullUpdatable for Term {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}
