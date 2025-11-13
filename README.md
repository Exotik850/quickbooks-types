# QuickBooks Online Types for Rust

The `quickbooks-types` crate provides strongly typed Rust models for QuickBooks Online (QBO) API entities and report types. It focuses on data structures, validation helpers, and report parameter builders. Network access and API calls are intentionally out of scope—use your preferred HTTP client alongside these types.

- Crate: quickbooks-types
- Version: 0.1.1
- License: MIT

QuickBooks API docs: https://developer.intuit.com/app/developer/qbo/docs/get-started

---

## Status and scope

This crate:

- Defines data models for common QBO entities (e.g., Invoice, Customer, Vendor, Item, etc.)
- Provides traits that capture operation preconditions such as can_create, can_full_update, can_sparse_update, etc.
- Includes helper types for lines, references, taxes, contact info, and metadata
- Implements a reports module with rich report parameter builders and types
- Offers optional ergonomics via a builder feature, and an optional reports+Polars integration feature

This crate does not:

- Make HTTP requests
- Provide an async runtime or HTTP client
- Offer a general SQL-like query builder for QBO entities

---

## Installation

~~~toml
[dependencies]
quickbooks-types = "0.1.1"
~~~

Optional features:

- builder: derive Builder types and add convenience constructors for top-level entities
- polars: enable report parsing helpers that integrate with Polars (exposes `reports::QBPolarsError`)

Examples:

~~~toml
[dependencies]
quickbooks-types = { version = "0.1.1", features = ["builder"] }
# or
quickbooks-types = { version = "0.1.1", features = ["polars"] }
~~~

---

## Modules and re-exports

- Entities re-exported at crate root:
  - Account, Attachable, Bill, BillPayment, CompanyInfo, Customer, Employee, Estimate, Invoice, Item, Payment, Preferences, SalesReceipt, Vendor
- Nested modules:
  - `common`: shared value types (e.g., `NtRef`, `MetaData`, contact and address types, taxes)
  - `reports`: report data structures and parameter builders
    - `reports::types`: report enum-like structs and `<Report>Params`
    - `reports::params`: reusable parameter enums and typed IDs
    - `reports::models`: report models (e.g., `Report`, `Row`, `ColData`)
    - `reports::polars` (feature = "polars"): Polars integration

---

## Quick start: Entities

Basic construction and serialization of an invoice:

~~~rust
use chrono::NaiveDate;
use serde_json;
use quickbooks_types::{Invoice, Line, LineDetail, SalesItemLineDetail};
use quickbooks_types::common::NtRef;

// minimal invoice with one sales item line
let invoice = Invoice {
    customer_ref: Some(NtRef::from(("John Doe", "CUST-123"))),
    txn_date: NaiveDate::from_ymd_opt(2024, 10, 1),
    line: Some(vec![
        Line {
            amount: Some(100.0),
            line_detail: LineDetail::SalesItemLineDetail(SalesItemLineDetail {
                item_ref: Some(NtRef::from(("Widget A", "ITEM-001"))),
                qty: Some(1.0),
                unit_price: Some(100.0),
                ..Default::default()
            }),
            ..Default::default()
        }
    ]),
    ..Default::default()
};

let json = serde_json::to_string_pretty(&invoice).unwrap();
println!("{json}");
~~~

Operation checks (traits):

~~~rust
use quickbooks_types::{Invoice, QBCreatable, QBReadable, QBDeletable, QBFullUpdatable};

let mut invoice = Invoice::default();
assert!(!invoice.can_create()); // missing required fields (customer_ref + line)

invoice.customer_ref = Some(("John Doe", "CUST-123").into());
invoice.line = Some(vec![]);
assert!(!invoice.can_create()); // line must be a non-empty collection with amounts

// simulate entity read from QBO
invoice.id = Some("123".into());
invoice.sync_token = Some("2".into());
assert!(invoice.can_read());          // ID is set
assert!(!invoice.can_full_update());  // still fails creation rules

// deletion requires "has_read" (ID + sync_token)
assert!(invoice.can_delete());
~~~

Convert entities to references (`NtRef`):

~~~rust
use quickbooks_types::{Customer, QBToRef};

let mut customer = Customer::default();
customer.id = Some("CUST-123".into());
customer.display_name = Some("John Doe".into());

let customer_ref = customer.to_ref().unwrap();
// customer_ref.name == Some("John Doe"), customer_ref.value == Some("CUST-123")
~~~

---

## Builder feature

Enable the `builder` feature to get derived builders for most entity types and a convenience `::new()` associated function for top-level entities (e.g., `Invoice::new()` returns an `InvoiceBuilder`).

~~~toml
[dependencies]
quickbooks-types = { version = "0.1.1", features = ["builder"] }
~~~

~~~rust
use chrono::NaiveDate;
use quickbooks_types::common::NtRef;
use quickbooks_types::{Invoice, LineDetail, QBCreatable};
use quickbooks_types::{LineBuilder, SalesItemLineDetailBuilder};

let invoice = Invoice::new()
    .customer_ref(NtRef::from(("John Doe", "CUST-123")))
    .txn_date(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap())
    .line(vec![
        LineBuilder::default()
            .amount(100.0)
            .line_detail(LineDetail::SalesItemLineDetail(
                SalesItemLineDetailBuilder::default()
                    .item_ref(NtRef::from(("Widget A", "ITEM-001")))
                    .qty(1.0)
                    .unit_price(100.0)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap(),
    ])
    .build()
    .unwrap();

assert!(invoice.can_create());
~~~

Attachable convenience (builder):

~~~rust
use quickbooks_types::Attachable;

#[cfg(feature = "builder")]
{
    use quickbooks_types::AttachableBuilder;

    // AttachableBuilder::file(...) sets file_path, derives file_name and content_type from extension
    let attachable = Attachable::new()
        .note("Invoice backup")
        // .file(&std::path::Path::new("docs/invoice.pdf"))?
        .build();

    assert!(attachable.is_ok());
}
~~~

---

## Reports

The `reports` module provides:

- Richly-typed report models: `reports::models::{Report, Row, ColData, ...}`
- Strongly-typed parameter builders per report: `reports::types::<Report>Params`
- Common enums and ID wrappers for parameters: `reports::params::*`

Build a query string for a report:

~~~rust
use chrono::NaiveDate;
use quickbooks_types::reports::types::*;
use quickbooks_types::reports::params::*;

let balance_sheet = BalanceSheetParams::new()
    .accounting_method(AccountingMethod::Cash)
    .start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
    .end_date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap())
    .date_macro(DateMacro::ThisFiscalYear)
    .summarize_column_by(SummarizeColumnBy::Month);

println!("{}", balance_sheet.to_query_string());
// -> accounting_method=Cash&start_date=2024-01-01&end_date=2024-12-31&date_macro=This Fiscal Year&summarize_column_by=Month
~~~

Another example:

~~~rust
use chrono::NaiveDate;
use quickbooks_types::reports::types::*;
use quickbooks_types::reports::params::*;

let ap_aging = APAgingDetailParams::new()
    .as_of_date(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())
    .aging_method(AgingMethod::Current)
    .vendor(VendorId(789))
    .column("Name")
    .column("DueDate")
    .column("Amount");

println!("{}", ap_aging.to_query_string());
~~~

Parsing a report result:

~~~rust
use quickbooks_types::reports::models::Report;

let json = r#"{
  "Header": { "ReportName": "BalanceSheet" },
  "Columns": { "Column": [ { "ColTitle": "Total", "ColType": "Money" } ] },
  "Rows": { "Row": [] }
}"#;

let report: Report = serde_json::from_str(json).unwrap();
assert_eq!(report.name(), Some("BalanceSheet"));
~~~

Polars integration (feature = "polars"):

- Enables `reports::polars` helpers and `reports::QBPolarsError`
- Provide your own conversion to Polars DataFrames depending on your use case

---

## Entities included

Top-level entities for which `QBItem` is implemented and a `Display` impl is provided:

- Account, Attachable, Bill, BillPayment, CompanyInfo, Customer, Employee, Estimate, Invoice, Item, Payment, Preferences, SalesReceipt, Vendor

Supporting value types (non-exhaustive):

- `common`: `NtRef`, `MetaData`, `Addr`, `Email`, `PhoneNumber`, `WebAddr`, `TxnTaxDetail`, `LinkedTxn`, and others
- `line`: `Line`, `LineDetail` and its variants (e.g., `SalesItemLineDetail`, `SubTotalLineDetail`, etc.), `LineField` (alias for `Vec<Line>`)

Operation traits (selected):

- `QBCreatable`: `can_create(&self) -> bool` – entity meets required fields for creation
- `QBReadable`: `can_read(&self) -> bool` – true if ID is present
- `QBDeletable`: `can_delete(&self) -> bool` – default requires both ID and sync_token
- `QBFullUpdatable`: `can_full_update(&self) -> bool` – type-specific requirements
- `QBSparseUpdateable`: `can_sparse_update(&self) -> bool` – typically `can_full_update()` + `sparse == true`
- `QBVoidable`: `can_void(&self) -> bool` – default requires both ID and sync_token
- `QBSendable` / `QBPDFable`: marker traits for entities that can be emailed / retrieved as PDF in QBO
- `QBToRef`: `to_ref(&self) -> Result<NtRef, QBTypeError>` – convert to reference usable in other entities

Note: These methods validate local preconditions only; they do not contact QBO.

---

## Errors

~~~rust
use quickbooks_types::QBTypeError;

#[derive(Debug, thiserror::Error)]
pub enum QBTypeError {
    #[cfg(feature = "builder")]
    #[error("Error validating QB Object in builder: {0}")]
    ValidationError(String),

    #[error("Missing field in QB Object: {0}")]
    MissingField(&'static str),

    #[error("QB Item could not be referenced!")]
    QBToRefError,
}
~~~

---

## Tips

- IDs and `sync_token` are required for delete and void operations (`has_read()` check)
- Builders (feature = "builder") return `Result<_, QBTypeError>` on `build()`
- Some enums contain placeholder or minimal variants where the QBO surface is large; open an issue or PR to extend as needed

---

## Contributing

Issues and PRs are welcome. If you need additional entities, fields, or extended report handling, feel free to propose a change.

---

## Documentation

API documentation is available at [docs.rs/quickbooks-types](https://docs.rs/quickbooks-types).

---

## License

MIT. See the LICENSE file for details.
