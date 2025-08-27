use chrono::NaiveDate;
use quickbooks_types::reports::params::*;
use quickbooks_types::reports::types::*;

fn main() {
    let balance_sheet = BalanceSheetParams::new()
        .accounting_method(AccountingMethod::Cash)
        .start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        .end_date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap())
        .date_macro(DateMacro::ThisFiscalYear)
        .summarize_column_by(SummarizeColumnBy::Month)
        .customer(CustomerId(123u32))
        .vendor(VendorId(456u32));

    println!("Query string: {}", balance_sheet.to_query_string());

    // Another example with different types
    let ap_aging = APAgingDetailParams::new()
        .as_of_date(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())
        .aging_method(AgingMethod::Current)
        .vendor(VendorId(789u32))
        .column("Name")
        .column("DueDate")
        .column("Amount");

    println!("AP Aging query: {}", ap_aging.to_query_string());
}
