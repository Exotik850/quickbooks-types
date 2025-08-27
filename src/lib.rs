// #![warn(clippy::pedantic)]

#[cfg(feature = "builder")]
#[macro_use]
extern crate derive_builder;

mod error;
mod models;
pub mod reports;
use std::fmt::{Debug, Display};

pub use error::*;
use models::common::{MetaData, NtRef};
pub use models::*;
use serde::{de::DeserializeOwned, Serialize};

/// Core trait for all QuickBooks entities.
///
/// This trait defines the fundamental interface that all QuickBooks entities must implement.
/// It provides access to common fields like ID, sync token, and metadata that are present
/// on all QuickBooks objects, as well as type information for API operations.
///
/// # Required Methods
///
/// - `id()`: Returns the entity's unique identifier
/// - `clone_id()`: Returns a cloned copy of the ID
/// - `sync_token()`: Returns the synchronization token for updates
/// - `meta_data()`: Returns metadata about the entity
/// - `name()`: Returns the entity type name for API calls
/// - `qb_id()`: Returns the lowercase entity identifier for URLs
///
/// # Default Methods
///
/// - `has_read()`: Returns true if the entity has both ID and sync token (indicates it was read from QB)
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{QBItem, Customer};
///
/// let customer = Customer::default();
///
/// // Check if entity has been read from QuickBooks
/// if customer.has_read() {
///     println!("Customer ID: {:?}", customer.id());
///     println!("Sync Token: {:?}", customer.sync_token());
/// }
///
/// // Get type information
/// println!("Entity name: {}", Customer::name()); // "Customer"
/// println!("API identifier: {}", Customer::qb_id()); // "customer"
/// ```
pub trait QBItem: Serialize + Default + Clone + Sized + DeserializeOwned + Debug + Send {
    fn id(&self) -> Option<&String>;
    fn clone_id(&self) -> Option<String>;
    fn sync_token(&self) -> Option<&String>;
    fn meta_data(&self) -> Option<&MetaData>;
    fn name() -> &'static str;
    fn qb_id() -> &'static str;
    fn has_read(&self) -> bool {
        self.id().is_some() && self.sync_token().is_some()
    }
}

macro_rules! impl_qb_data {
    ($($x:ident),+) => {
        $(
            #[cfg(feature="builder")]
            paste::paste! {
                #[allow(clippy::new_ret_no_self)]
                impl [<$x>] {
                    #[must_use] pub fn new() -> [<$x Builder>] {
                        [<$x Builder>]::default()
                    }
                }
            }

            impl QBItem for $x {
                fn id(&self) -> Option<&String> {
                    self.id.as_ref()
                }

                fn clone_id(&self) -> Option<String> {
                    self.id.clone()
                }

                fn sync_token(&self) -> Option<&String> {
                    self.sync_token.as_ref()
                }

                fn meta_data(&self) -> Option<&MetaData> {
                    self.meta_data.as_ref()
                }

                #[inline]
                fn name() -> &'static str {
                    stringify!($x)
                }

                #[inline]
                fn qb_id() -> &'static str {
                    paste::paste! {
                        stringify!([<$x:lower>])
                    }
                }
            }

            impl Display for $x {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} : {}", Self::name(), serde_json::to_string_pretty(self).expect("Could not serialize object for display!"))
                }
            }
        )+
   }
}

impl_qb_data!(
    Invoice,
    Vendor,
    Payment,
    Item,
    Estimate,
    Employee,
    Customer,
    CompanyInfo,
    Bill,
    Attachable,
    Account,
    Preferences,
    SalesReceipt,
    BillPayment
);

/// Trait for entities that can be created in QuickBooks.
///
/// This trait defines the validation logic for determining whether an entity
/// has the required fields to be successfully created via the QuickBooks API.
/// Each entity implements its own validation rules based on QuickBooks requirements.
///
/// # Required Methods
///
/// - `can_create()`: Returns true if the entity has all required fields for creation
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBCreatable};
///
/// let mut customer = Customer::default();
///
/// // Check if customer can be created (will be false - missing required fields)
/// assert!(!customer.can_create());
///
/// // Add required field
/// customer.display_name = Some("John Doe".to_string());
///
/// // Now it can be created
/// assert!(customer.can_create());
/// ```
///
/// # Implementation Notes
///
/// Different entities have different creation requirements:
/// - **Customer/Vendor**: Requires display_name or individual name components
/// - **Account**: Requires name and account_type or account_sub_type
/// - **Invoice**: Requires customer reference and line items
/// - **Item**: Requires name and type
pub trait QBCreatable {
    fn can_create(&self) -> bool;
}

/// Trait for entities that can be read from QuickBooks by ID.
///
/// This trait is automatically implemented for all [`QBItem`] types and provides
/// the ability to read entities from QuickBooks using their unique identifier.
///
/// # Default Implementation
///
/// The default implementation checks if the entity has an ID field, which is
/// required for read operations.
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBReadable};
///
/// let mut customer = Customer::default();
/// customer.id = Some("123".to_string());
///
/// // Can read because it has an ID
/// assert!(customer.can_read());
/// ```
pub trait QBReadable: QBItem {
    fn can_read(&self) -> bool;
}

impl<T: QBItem> QBReadable for T {
    fn can_read(&self) -> bool {
        self.id().is_some()
    }
}

/// Trait for entities that can be queried from QuickBooks.
///
/// This trait is automatically implemented for all [`QBItem`] types and indicates
/// that the entity supports QuickBooks SQL-like query operations.
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBQueryable};
///
/// // All QBItem types automatically implement QBQueryable
/// let customer = Customer::default();
/// ```
pub trait QBQueryable: QBItem {}
impl<T: QBItem> QBQueryable for T {}

/// Trait for entities that can be deleted from QuickBooks.
///
/// This trait is automatically implemented for all [`QBItem`] types and provides
/// validation for delete operations. Entities must have been read from QuickBooks
/// (have both ID and sync token) to be deletable.
///
/// # Default Implementation
///
/// The default implementation uses [`QBItem::has_read()`] to verify the entity
/// has both an ID and sync token, which are required for delete operations.
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBDeletable};
///
/// let mut customer = Customer::default();
/// customer.id = Some("123".to_string());
/// customer.sync_token = Some("2".to_string());
///
/// // Can delete because it has both ID and sync token
/// assert!(customer.can_delete());
/// ```
pub trait QBDeletable: QBItem {
    fn can_delete(&self) -> bool {
        self.has_read()
    }
}

/// Trait for entities that can be voided in QuickBooks.
///
/// Voiding is a special operation in QuickBooks that marks transactions as void
/// while preserving them for audit purposes. Only certain entities support voiding.
///
/// # Default Implementation
///
/// The default implementation requires that the entity has been read from QuickBooks
/// (has both ID and sync token).
///
/// # Supported Entities
///
/// Typically includes: Invoice, Payment, Bill, Check, SalesReceipt, and other transactional entities.
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Invoice, QBVoidable};
///
/// let mut invoice = Invoice::default();
/// invoice.id = Some("123".to_string());
/// invoice.sync_token = Some("2".to_string());
///
/// // Can void because it has been read from QuickBooks
/// assert!(invoice.can_void());
/// ```
pub trait QBVoidable: QBItem {
    fn can_void(&self) -> bool {
        self.has_read()
    }
}

/// Trait for entities that support full update operations.
///
/// Full updates require sending the complete entity data to QuickBooks,
/// replacing all fields with the provided values. This is in contrast to
/// sparse updates which only update specified fields.
///
/// # Required Methods
///
/// - `can_full_update()`: Returns true if the entity can be fully updated
///
/// # Implementation Notes
///
/// Typically requires:
/// - Entity has been read from QuickBooks (has ID and sync token)
/// - Entity meets creation requirements (has required fields)
/// - Some entities may have additional validation rules
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBFullUpdatable};
///
/// let mut customer = Customer::default();
/// customer.id = Some("123".to_string());
/// customer.sync_token = Some("2".to_string());
/// customer.display_name = Some("John Doe".to_string());
///
/// // Check if can be fully updated
/// if customer.can_full_update() {
///     // Proceed with full update
/// }
/// ```
pub trait QBFullUpdatable {
    fn can_full_update(&self) -> bool;
}

/// Trait for entities that support sparse update operations.
///
/// Sparse updates allow updating only specific fields of an entity without
/// affecting other fields. This is more efficient and safer than full updates
/// when you only need to change specific values.
///
/// # Required Methods
///
/// - `can_sparse_update()`: Returns true if the entity can be sparse updated
///
/// # Implementation Notes
///
/// Typically requires:
/// - Entity can perform full updates
/// - Entity has the `sparse` field set to `true`
/// - QuickBooks API supports sparse updates for this entity type
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBSparseUpdateable};
///
/// let mut customer = Customer::default();
/// customer.id = Some("123".to_string());
/// customer.sync_token = Some("2".to_string());
/// customer.display_name = Some("John Doe".to_string());
/// customer.sparse = Some(true);
///
/// // Check if can be sparse updated
/// if customer.can_sparse_update() {
///     // Proceed with sparse update
/// }
/// ```
pub trait QBSparseUpdateable {
    fn can_sparse_update(&self) -> bool;
}

/// Trait for entities that can be sent via email from QuickBooks.
///
/// This trait marks entities that support QuickBooks' built-in email functionality,
/// such as sending invoices or estimates to customers via email.
///
/// # Supported Entities
///
/// Typically includes: Invoice, Estimate, SalesReceipt, and other customer-facing documents.
pub trait QBSendable {}

/// Trait for entities that can be generated as PDF documents.
///
/// This trait marks entities that support QuickBooks' PDF generation functionality,
/// allowing you to retrieve formatted PDF versions of documents.
///
/// # Supported Entities
///
/// Typically includes: Invoice, Estimate, SalesReceipt, Statement, and other printable documents.
pub trait QBPDFable {}

/// Trait for entities that can be converted to QuickBooks entity references.
///
/// Entity references (`NtRef`) are used throughout QuickBooks to link entities together.
/// For example, an invoice has a customer reference that points to a specific customer.
///
/// # Required Methods
///
/// - `to_ref()`: Converts the entity to an `NtRef` for use in other entities
///
/// # Returns
///
/// Returns a `Result<NtRef, QBTypeError>` where:
/// - `Ok(NtRef)` if the entity can be referenced (has ID and name field)
/// - `Err(QBTypeError::QBToRefError)` if the entity cannot be referenced
///
/// # Examples
///
/// ```rust
/// use quickbooks_types::{Customer, QBToRef};
///
/// let mut customer = Customer::default();
/// customer.id = Some("123".to_string());
/// customer.display_name = Some("John Doe".to_string());
///
/// // Convert to reference for use in other entities
/// let customer_ref = customer.to_ref()?;
///
/// // Use the reference in an invoice
/// let mut invoice = Invoice::default();
/// invoice.customer_ref = Some(customer_ref);
/// ```
pub trait QBToRef: QBItem {
    fn to_ref(&self) -> Result<NtRef, QBTypeError>;
}

macro_rules! impl_qb_to_ref {
  ($($struct:ident {$name_field:ident}),+) => {
    $(
      impl QBToRef for $struct {
        fn to_ref(&self) -> Result<NtRef, $crate::QBTypeError> {
          if self.id.is_some() {
            Ok(NtRef {
              entity_ref_type: Some(Self::name().into()),
              name: self.$name_field.clone(),
              value: self.id.clone()
            })
          } else {
            Err($crate::QBTypeError::QBToRefError)
          }
        }
      }
    )+
  }
}

impl_qb_to_ref!(
    Account {
        fully_qualified_name
    },
    Attachable { file_name },
    Invoice { doc_number },
    SalesReceipt { doc_number },
    Item { name },
    Customer { display_name },
    Vendor { display_name }
);

/*
Create: ✓
- Account
- Attachable
- Bill
- Customer
- Employee
- Estimate
- Invoice
- Item (Category)
- Payment
- Sales Receipt
- Vendor
Read: ✓
- Attachable
- Account
- Bill
- CompanyInfo
- Customer
- Employee
- Estimate
- Invoice
- Item (Category, Bundle)
- Preferences
- Sales Receipt
- Vendor
Query: ✓
- Attachable
- Account
- Bill
- CompanyInfo
- Customer
- Employee
- Estimate
- Invoice
- Item (Category, Bundle)
- Payment
- Preferences
- Sales Receipt
- Vendor
Delete: ✓
- Attachable
- Bill
- Estimate
- Invoice
- Payment
- Sales Receipt
Void: ✓
- Invoice
- Payment
- Sales Receipt
Full Update: ✓
- Account
- Attachable
- Bill
- CompanyInfo
- Customer
- Employee
- Estimate
- Invoice
- Item (Category)
- Payment
- Preferences
- Sales Receipt
- Vendor
Sparse Update: ✓
- CompanyInfo
- Customer
- Estimate
- Invoice
- Sales Receipt
Send: ✓
- Estimate
- Invoice
- Payment
- Sales Receipt
Get as PDF: ✓
- Estimate
- Invoice
- Payment
- Sales Receipt

- Attachment has three other actions that are unique
- Upload ✓

*/
