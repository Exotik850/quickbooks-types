use std::borrow::Cow;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

macro_rules! impl_display_enum {
    (
      $($doc:literal)?
      $name:ident => {
        $(
            $variant_display:ident;
        )* -
        $(
            $variant:ident => $display:expr
        ),* $(,)?
      }
    ) => {
        $(#[doc = $doc])?
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            $(
                $variant_display,
            )*
            $(
                #[serde(rename = $display)]
                $variant,
            )*
        }

        impl $name {
            /// Returns the string representation of the enum variant.
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        $name::$variant_display => stringify!($variant_display),
                    )*
                    $(
                        $name::$variant => $display,
                    )*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl HasValue for $name {
            fn value(&self) -> Cow<'_, str> {
                self.as_str().into()
            }
        }
    };
}

impl_display_enum!(
  "Represents various predefined date ranges for reports."
  DateMacro => {
  All;
  Today;
  Yesterday;
  -
  ThisWeek => "This Week",
  ThisWeekToDate => "This Week-to-date",
  ThisMonth => "This Month",
  ThisMonthToDate => "This Month-to-date",
  ThisFiscalQuarter => "This Fiscal Quarter",
  ThisFiscalQuarterToDate => "This Fiscal Quarter-to-date",
  ThisFiscalYear => "This Fiscal Year",
  ThisFiscalYearToDate => "This Fiscal Year-to-date",
  ThisCalendarQuarter => "This Calendar Quarter",
  ThisCalendarQuarterToDate => "This Calendar Quarter-to-date",
  ThisCalendarYear => "This Calendar Year",
  ThisCalendarYearToDate => "This Calendar Year-to-date",
  LastWeek => "Last Week",
  LastWeekToDate => "Last Week-to-date",
  LastMonth => "Last Month",
  LastMonthToDate => "Last Month-to-date",
  LastFiscalQuarter => "Last Fiscal Quarter",
  LastFiscalQuarterToDate => "Last Fiscal Quarter-to-date",
  LastFiscalYear => "Last Fiscal Year",
  LastFiscalYearToDate => "Last Fiscal Year-to-date",
  LastCalendarQuarter => "Last Calendar Quarter",
  LastCalendarQuarterToDate => "Last Calendar Quarter-to-date",
  LastCalendarYear => "Last Calendar Year",
  LastCalendarYearToDate => "Last Calendar Year-to-date",
  NextWeek => "Next Week",
  Next4Weeks => "Next 4 Weeks",
  NextMonth => "Next Month",
  NextFiscalQuarter => "Next Fiscal Quarter",
  NextFiscalYear => "Next Fiscal Year",
  NextCalendarQuarter => "Next Calendar Quarter",
  NextCalendarYear => "Next Calendar Year"
});

impl_display_enum!(
  "Represents how columns are summarized in reports."
  SummarizeColumnBy => {
    Total;
    Year;
    Quarter;
    FiscalYear;
    FiscalQuarter;
    Month;
    Week;
    Days;
    Customers;
    Vendors;
    Employees;
    Departments;
    Classes;
    ProductsAndServices;
    -
});
impl_display_enum!(
  "Represents different aging methods for reports."
  AgingMethod => {
    Current;
    -
    ReportDate => "Report_Date",
});

impl_display_enum!(
  "Represents different accounting methods for reports."
  AccountingMethod => {
    Cash;
    Accrual;
    -
});

impl_display_enum!(
  "Represents different sort orders for reports."
  SortOrder => {
    Ascending;
    Descending;
    -
});

impl_display_enum!(
  "Represents Paid status for AR reports."
  ArPaid => {
    All;
    Paid;
    Unpaid;
    -
});

impl_display_enum!(
  "Represents different attachment types for reports."
  AttachmentType => {
    -
    TemporaryLinks => "TEMPORARY_LINKS",
    None => "NONE",
});

impl_display_enum!(
  "Filters report contents to include information for specified check status."
  Cleared => {
    Cleared;
    Uncleared;
    Reconciled;
    Deposited;
    -
});

impl_display_enum!(
  "Represents whether a report was printed."
  Printed => {
    Printed;
    -
    ToBePrinted => "To_be_printed",
});

macro_rules! impl_id_param {
    ($($name:ident),*) => {

      $(
          paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[doc = "Represents the ID for a " $name " in QuickBooks reports."]
            pub struct [<$name Id>](pub u32);
            impl HasValue for [<$name Id>] {
                fn value(&self) -> Cow<'_, str> {
                    self.to_string().into()
                }
            }

            impl std::fmt::Display for [<$name Id>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
          }
        )*
    };
}

impl_id_param!(Customer, Vendor, Employee, Item, Class, Department, Account, Term);

pub trait HasValue {
    // fn name() -> &'static str;
    fn value<'a>(&'a self) -> Cow<'a, str>;
}

// Implement QBReportParam directly for common types
impl HasValue for String {
    fn value(&self) -> Cow<'_, str> {
        self.into()
    }
}

impl HasValue for &str {
    fn value(&self) -> Cow<'_, str> {
        (*self).into()
    }
}

impl HasValue for u32 {
    fn value(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}

impl HasValue for NaiveDate {
    fn value(&self) -> Cow<'_, str> {
        self.format("%Y-%m-%d").to_string().into()
    }
}

impl HasValue for bool {
    fn value(&self) -> Cow<'_, str> {
        if *self { "true" } else { "false" }.into()
    }
}

impl<V: HasValue> HasValue for Vec<V> {
    fn value(&self) -> Cow<'_, str> {
        self.iter()
            .map(|v| v.value())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }
}
