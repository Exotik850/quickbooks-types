// #![warn(clippy::pedantic)]

#[cfg(feature = "builder")]
#[macro_use]
extern crate derive_builder;

mod error;
mod models;
pub mod reports;
use std::fmt::{Debug, Display};

use const_str::convert_ascii_case;
pub use error::*;
use models::common::{MetaData, NtRef};
pub use models::*;
use serde::{de::DeserializeOwned, Serialize};

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
                    convert_ascii_case!(lower, stringify!($x))
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

pub trait QBCreatable {
    fn can_create(&self) -> bool;
}

pub trait QBReadable: QBItem {
    fn can_read(&self) -> bool;
}

impl<T: QBItem> QBReadable for T {
    fn can_read(&self) -> bool {
        self.id().is_some()
    }
}

pub trait QBQueryable: QBItem {}
impl<T: QBItem> QBQueryable for T {}

pub trait QBDeletable: QBItem {
    fn can_delete(&self) -> bool {
        self.has_read()
    }
}

pub trait QBVoidable: QBItem {
    fn can_void(&self) -> bool {
        self.has_read()
    }
}

pub trait QBFullUpdatable {
    fn can_full_update(&self) -> bool;
}

pub trait QBSparseUpdateable {
    fn can_sparse_update(&self) -> bool;
}

pub trait QBSendable {}

pub trait QBPDFable {}

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
