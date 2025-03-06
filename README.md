# QuickBooks Types for Rust

<!-- [![Crates.io](https://img.shields.io/crates/v/quickbooks-types)](https://crates.io/crates/quickbooks-types)
[![Documentation](https://docs.rs/quickbooks-types/badge.svg)](https://docs.rs/quickbooks-types) -->
<!-- [![License](https://img.shields.io/crates/l/quickbooks-types)](LICENSE) -->

The `quickbooks-types` crate provides strongly-typed Rust representations of QuickBooks Online (QBO) API entities, enabling seamless integration with QuickBooks accounting software. It is designed to work with the [QuickBooks API](https://developer.intuit.com/app/developer/qbo/docs/get-started) and is ideal for building financial applications, integrations, and automation tools.

---

## Features

- **Complete Coverage**: Supports all major QuickBooks entities, including:
  - Accounts
  - Invoices
  - Customers
  - Vendors
  - Payments
  - Sales Receipts
  - Items
  - Bills
  - Estimates
  - Attachments
  - And more!
- **Strong Typing**: All entities are represented as Rust structs with proper types (e.g., `NaiveDate` for dates, `f64` for monetary values).
- **Validation**: Built-in validation for required fields and business rules.
- **Serialization/Deserialization**: Seamless integration with `serde` for JSON serialization and deserialization.
- **Builder Pattern**: Optional builder pattern for easy object creation.
- **Query Support**: Helper methods for constructing SQL-like queries for QuickBooks entities.
- **Error Handling**: Comprehensive error types for API and validation errors.
- **Async Support**: Designed for use with async Rust and popular HTTP clients like `reqwest`.

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
quickbooks-types = "0.1.0"
```

---

## Usage

### Basic Example

```rust
use quickbooks_types::{Invoice, Customer, Line, LineDetail, SalesItemLineDetail, NtRef};
use chrono::NaiveDate;

// Create a customer
let customer = Customer {
    id: Some("123".to_string()),
    display_name: Some("John Doe".to_string()),
    ..Default::default()
};

// Create an invoice
let invoice = Invoice {
    id: Some("456".to_string()),
    customer_ref: Some(NtRef {
        value: customer.id.clone(),
        name: customer.display_name.clone(),
        ..Default::default()
    }),
    line: Some(vec![
        Line {
            amount: Some(100.0),
            line_detail: LineDetail::SalesItemLineDetail(SalesItemLineDetail {
                item_ref: Some(NtRef {
                    value: Some("ITEM123".to_string()),
                    name: Some("Product A".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    ]),
    txn_date: Some(NaiveDate::from_ymd(2023, 10, 1)),
    ..Default::default()
};

// Serialize to JSON for API requests
let json = serde_json::to_string(&invoice).unwrap();
println!("{}", json);
```

### Querying Entities

```rust
use quickbooks_types::{Customer, QBQueryable};

// Query customers with a specific name
let query = Customer::query("WHERE DisplayName = 'John Doe'");
```

### Builder Pattern (Optional)

Enable the `builder` feature in `Cargo.toml`:

```toml
[dependencies]
quickbooks-types = { version = "0.1.0", features = ["builder"] }
```

Then use the builder pattern:

```rust
use quickbooks_types::{InvoiceBuilder, LineBuilder, SalesItemLineDetailBuilder};

let invoice = InvoiceBuilder::default()
    .id("123".to_string())
    .customer_ref(NtRef {
        value: Some("CUST123".to_string()),
        name: Some("John Doe".to_string()),
        ..Default::default()
    })
    .line(vec![
        LineBuilder::default()
            .amount(100.0)
            .line_detail(LineDetail::SalesItemLineDetail(
                SalesItemLineDetailBuilder::default()
                    .item_ref(NtRef {
                        value: Some("ITEM123".to_string()),
                        name: Some("Product A".to_string()),
                        ..Default::default()
                    })
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap(),
    ])
    .build()
    .unwrap();
```

---

## Supported Entities

The crate provides types for the following QuickBooks entities:

- **Accounting**:
  - Account
  - Budget
  - Class
  - Department
  - TaxCode
  - TaxRate
- **Sales**:
  - Invoice
  - Estimate
  - SalesReceipt
  - Payment
  - CreditMemo
- **Purchases**:
  - Bill
  - VendorCredit
  - PurchaseOrder
- **Customers & Vendors**:
  - Customer
  - Vendor
- **Items**:
  - Item
  - Inventory
  - NonInventory
  - Service
- **Payroll**:
  - Employee
  - TimeActivity
- **Attachments**:
  - Attachable
- **Reports**:
  - BalanceSheet
  - ProfitAndLoss
  - TrialBalance

---

## Error Handling

The crate provides a comprehensive error type `QBTypeError` for handling validation and API errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum QBTypeError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("API error: {0}")]
    ApiError(String),
    // ...
}
```

---

## Optional Features

- **builder**: Enables the builder pattern for entity creation.
- **serde**: Enables serialization and deserialization (enabled by default).
- **chrono**: Enables date/time support (enabled by default).

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or additional features.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Documentation

For detailed documentation, visit [docs.rs/quickbooks-types](https://docs.rs/quickbooks-types).
