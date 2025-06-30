use std::borrow::Cow;

use super::params::*;
use chrono::NaiveDate;

/// Represents parameters for QuickBooks reports.
pub trait QBReportParams {
    fn params(&self) -> impl Iterator<Item = (&'static str, Cow<str>)> + '_;
    fn to_query_string(&self) -> String {
        self.params()
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<_>>()
            .join("&")
    }
}

/// Represents a type of QuickBooks report.
pub trait QBReportType {
    type QueryParams: QBReportParams;
    fn url_name(&self) -> &'static str;
    // fn valid_query_params() -> &'static [&'static str];
}

use paste::paste;

macro_rules! impl_report_type {
    ($(
      $report_ty:ident, $url_name:expr, [$($param:tt),* $(,)?]; $(($doc:expr))?
    )*
    $(;)?) => {
      $(

        paste! {
          #[doc = "Type to represent the `" $report_ty "` report in quickbooks:\n https://developer.intuit.com/app/developer/qbo/docs/api/accounting/report-entities/" [<$report_ty:lower>] "\n\n" $($doc)?]
          pub struct $report_ty;

          impl QBReportType for $report_ty {
              type QueryParams = [<$report_ty Params>];
              fn url_name(&self) -> &'static str {
                  $url_name
              }
          }

          #[derive(Debug, Default)]
          #[allow(non_snake_case)]
          #[doc = "Parameters for the `" $report_ty "` report.\n\n" $($doc)?]
          pub struct [<$report_ty Params>] {
              $(
                pub $param: Option<impl_report_type!(@param_type $param)>,
              )*
          }

          impl [<$report_ty Params>] {
              pub fn new() -> Self {
                  Self {
                      $(
                        $param: None,
                      )*
                  }
              }

              $(
                pub fn $param(mut self, param: impl Into<impl_report_type!(@param_type $param)>) -> Self {
                    self.$param = Some(param.into());
                    self
                }
              )*

              fn iter_params(&self) -> impl Iterator<Item = (&'static str, Option<Cow<str>>)> {
                  [
                    $(
                      (stringify!($param), self.$param.as_ref().map(|p| p.value())),
                    )*
                  ]
                  .into_iter()
              }
          }

          impl QBReportParams for [<$report_ty Params>] {
              fn params(&self) -> impl Iterator<Item = (&'static str, Cow<str>)> + '_ {
                  self.iter_params().filter_map(|(name, value)| {
                      value.map(|v| (name, v.into()))
                  })
              }
          }
        }
      )+
    };

    // Helper macro to map parameter names to appropriate types
    // TODO : Ensure all of these types are strongly-typed where possible
    (@param_type accounting_method) => { AccountingMethod };
    (@param_type date_macro) => { DateMacro };
    (@param_type start_date) => { NaiveDate };
    (@param_type end_date) => { NaiveDate };
    (@param_type summarize_column_by) => { SummarizeColumnBy };
    (@param_type as_of_date) => { NaiveDate };
    (@param_type aging_method) => { AgingMethod };
    (@param_type vendor) => { VendorId };
    (@param_type customer) => { CustomerId };
    (@param_type columns) => { String };
    (@param_type qzurl) => { String };
    (@param_type department) => { String };
    (@param_type report_date) => { NaiveDate };
    (@param_type sort_order) => { SortOrder };
    (@param_type shipvia) => { String };
    (@param_type term) => { TermId };
    (@param_type end_duedate) => { NaiveDate };
    (@param_type start_duedate) => { NaiveDate };
    (@param_type custom1) => { String };
    (@param_type custom2) => { String };
    (@param_type custom3) => { String };
    (@param_type num_periods) => { u32 };
    (@param_type past_due) => { String };
    (@param_type aging_period) => { String };
    (@param_type adjusted_gain_loss) => { String };
    (@param_type class) => { String };
    (@param_type item) => { ItemId };
    (@param_type sort_by) => { String };
    (@param_type arpaid) => { ArPaid };
    (@param_type attachment_type) => { AttachmentType };
    (@param_type with_qbo_identifier) => { bool };
    (@param_type add_due_date) => { String };
    (@param_type account) => { AccountId };
    (@param_type source_account) => { AccountId };
    (@param_type account_type) => { String };
    (@param_type end_svcdate) => { NaiveDate };
    (@param_type svcdate_macro) => { String };
    (@param_type start_svcdate) => { NaiveDate };
    (@param_type group_by) => { String };
    (@param_type payment_method) => { String };
    (@param_type employee) => { String };
    (@param_type agency_id) => { String };
    (@param_type duedate_macro) => { String };
    (@param_type bothamount) => { String };
    (@param_type transaction_type) => { String };
    (@param_type docnum) => { String };
    (@param_type start_moddate) => { NaiveDate };
    (@param_type source_account_type) => { String };
    (@param_type start_createdate) => { NaiveDate };
    (@param_type memo) => { String };
    (@param_type appaid) => { String };
    (@param_type moddate_macro) => { String };
    (@param_type printed) => { Printed };
    (@param_type createdate_macro) => { String };
    (@param_type cleared) => { Cleared };
    (@param_type end_createdate) => { NaiveDate };
    (@param_type name) => { String };
    (@param_type end_moddate) => { NaiveDate };

    (@param_type $param:tt) => { compile_error!(
        "Unsupported parameter type for report: {}",
        stringify!($param)
    ) };

    () => {}
}

impl_report_type!(
  AccountListDetail, "AccountList", [
      accounting_method,
      date_macro,
      start_date,
      end_date,
      summarize_column_by
  ]; ("List of accounts with details")

  APAgingDetail, "AgedPayableDetail", [
    as_of_date,
    aging_method,
    vendor,
    columns
  ];

  APAgingSummary, "AgedPayables", [
    customer,
    qzurl,
    vendor,
    date_macro,
    department,
    report_date,
    sort_order,
    aging_method
  ];

  ARAgingDetail, "AgedReceivableDetail", [
    customer,
    shipvia,
    term,
    end_duedate,
    start_duedate,
    custom1,
    custom2,
    custom3,
    report_date,
    num_periods,
    aging_method,
    past_due,
    aging_period,
    columns
  ];

  ARAgingSummary, "AgedReceivables", [
    customer,
    qzurl,
    date_macro,
    aging_method,
    report_date,
    sort_order,
    department
  ];

  BalanceSheet, "BalanceSheet", [
    customer,
    qzurl,
    end_date,
    accounting_method,
    date_macro,
    adjusted_gain_loss,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    vendor,
    start_date
  ];

  CashFlow, "CashFlow", [
    customer,
    vendor,
    end_date,
    date_macro,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    start_date
  ];

  CustomerBalance, "CustomerBalance", [
    customer,
    accounting_method,
    date_macro,
    arpaid,
    report_date,
    sort_order,
    summarize_column_by,
    department
  ];

  CustomerBalanceDetail, "CustomerBalanceDetail", [
    customer,
    shipvia,
    term,
    end_duedate,
    start_duedate,
    custom1,
    custom2,
    custom3,
    arpaid,
    report_date,
    sort_order,
    aging_method,
    department
  ];

  CustomerIncome, "CustomerIncome", [
    customer,
    term,
    accounting_method,
    end_date,
    date_macro,
    class,
    sort_order,
    summarize_column_by,
    department,
    start_date,
    vendor
  ];

  FECReport, "FECReport", [
    attachment_type,
    with_qbo_identifier,
    start_date,
    end_date,
    add_due_date
  ];

  GeneralLedger, "GeneralLedger", [
    customer,
    account,
    accounting_method,
    source_account,
    end_date,
    date_macro,
    account_type,
    sort_by,
    sort_order,
    start_date,
    summarize_column_by,
    class,
    item,
    department,
    vendor,
    columns
  ];

  GeneralLedgerFR, "GeneralLedgerFR", [
    customer,
    account,
    accounting_method,
    source_account,
    end_date,
    date_macro,
    account_type,
    sort_by,
    sort_order,
    start_date,
    summarize_column_by,
    class,
    vendor
  ];

  InventoryValuationDetail, "InventoryValuationDetail", [
    end_date,
    end_svcdate,
    date_macro,
    svcdate_macro,
    start_svcdate,
    group_by,
    start_date,
    columns
  ];

  InventoryValuationSummary, "InventoryValuationSummary", [
    qzurl,
    date_macro,
    item,
    report_date,
    sort_order,
    summarize_column_by
  ];

  JournalReport, "JournalReport", [
    end_date,
    date_macro,
    sort_by,
    sort_order,
    start_date,
    columns
  ];

  ProfitAndLoss, "ProfitAndLoss", [
    customer,
    qzurl,
    accounting_method,
    end_date,
    date_macro,
    adjusted_gain_loss,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    vendor,
    start_date
  ];

  ProfitAndLossDetail, "ProfitAndLossDetail", [
    customer,
    account,
    accounting_method,
    end_date,
    date_macro,
    adjusted_gain_loss,
    class,
    sort_by,
    payment_method,
    sort_order,
    employee,
    department,
    vendor,
    account_type,
    start_date,
    columns
  ];

  SalesByClassSummary, "ClassSales", [
    customer,
    accounting_method,
    end_date,
    date_macro,
    class,
    item,
    summarize_column_by,
    department,
    start_date
  ];

  SalesByCustomer, "CustomerSales", [
    customer,
    qzurl,
    accounting_method,
    end_date,
    date_macro,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    start_date
  ];

  SalesByDepartment, "DepartmentSales", [
    customer,
    accounting_method,
    end_date,
    date_macro,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    start_date
  ];

  SalesByProduct, "ItemSales", [
    customer,
    end_duedate,
    accounting_method,
    end_date,
    date_macro,
    start_duedate,
    class,
    item,
    sort_order,
    summarize_column_by,
    department,
    start_date
  ];

  TaxSummary, "TaxSummary", [
    agency_id,
    accounting_method,
    end_date,
    date_macro,
    sort_order,
    start_date
  ];

  TransactionList, "TransactionList", [
    date_macro,
    payment_method,
    duedate_macro,
    arpaid,
    bothamount,
    transaction_type,
    docnum,
    start_moddate,
    source_account_type,
    group_by,
    start_date,
    department,
    start_duedate,
    columns,
    end_duedate,
    vendor,
    end_date,
    memo,
    appaid,
    moddate_macro,
    printed,
    createdate_macro,
    cleared,
    customer,
    qzurl,
    term,
    end_createdate,
    name,
    sort_by,
    sort_order,
    start_createdate,
    end_moddate
  ];

  TransactionListByCustomer, "TransactionListByCustomer", [
    date_macro,
    payment_method,
    duedate_macro,
    arpaid,
    bothamount,
    transaction_type,
    docnum,
    start_moddate,
    source_account_type,
    group_by,
    start_date,
    department,
    start_duedate,
    columns,
    end_duedate,
    end_date,
    memo,
    appaid,
    moddate_macro,
    printed,
    createdate_macro,
    cleared,
    customer,
    qzurl,
    term,
    end_createdate,
    name,
    sort_by,
    sort_order,
    start_createdate,
    end_moddate
  ];

  TransactionListByVendor, "TransactionListByVendor", [
    date_macro,
    payment_method,
    duedate_macro,
    arpaid,
    bothamount,
    transaction_type,
    docnum,
    start_moddate,
    source_account_type,
    group_by,
    start_date,
    department,
    start_duedate,
    columns,
    end_duedate,
    vendor,
    end_date,
    memo,
    appaid,
    moddate_macro,
    printed,
    createdate_macro,
    cleared,
    qzurl,
    term,
    end_createdate,
    name,
    sort_by,
    sort_order,
    start_createdate,
    end_moddate
  ];

  TransactionListWithSplits, "TransactionListWithSplits", [
    docnum,
    name,
    end_date,
    date_macro,
    payment_method,
    source_account_type,
    transaction_type,
    group_by,
    sort_by,
    sort_order,
    start_date,
    columns
  ];

  TrialBalance, "TrialBalance", [
    accounting_method,
    end_date,
    date_macro,
    sort_order,
    summarize_column_by,
    start_date
  ];

  VendorBalance, "VendorBalance", [
    qzurl,
    accounting_method,
    date_macro,
    appaid,
    report_date,
    sort_order,
    summarize_column_by,
    department,
    vendor
  ];

  VendorBalanceDetail, "VendorBalanceDetail", [
    term,
    accounting_method,
    date_macro,
    appaid,
    report_date,
    sort_order,
    summarize_column_by,
    department,
    vendor,
    columns,
    duedate_macro,
    start_duedate,
    end_duedate
  ];

  VendorExpenses, "VendorExpenses", [
    customer,
    vendor,
    end_date,
    date_macro,
    class,
    sort_order,
    summarize_column_by,
    department,
    accounting_method,
    start_date
  ];
);
