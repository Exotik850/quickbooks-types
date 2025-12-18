use crate::common::TypedRef;

// we can represent the child -> parent relationship for now,
// parent -> children requires querying the API, which is out of scope here

/// Trait for "this object has a single linked object of type `Target`".
///
/// Returns an optional reference to the linked object.
pub trait LinkedTo<Target> {
    const FIELD: &'static str;
    fn linked(&self) -> Option<&TypedRef<Target>>;
}

/// Macro to implement `LinkedTo` for a struct with a linked field.
///
/// # Examples
/// ```rs
/// impl_linked!(Customer as primary_email_addr => Email);
/// ```
///
/// Generates:
/// ```rs
/// impl LinkedTo<Email> for Customer {
///     const FIELD: &'static str = "PrimaryEmailAddr";
///     fn linked(&self) -> Option<&TypedRef<Email>> {
///         self.primary_email_addr.as_ref()
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_linked {
    ($source:ty as $field:tt => $target:ty) => {
        ::paste::paste! {
            impl $crate::linked::LinkedTo<$target> for $source {
                const FIELD: &'static str = stringify!([<$field:camel>]);
                fn linked(&self) -> Option<&$crate::common::TypedRef<$target>> {
                    self.$field.as_ref()
                }
            }
        }
    };
    ($source:ty => $target:ty) => {
        ::paste::paste! {
            impl $crate::linked::LinkedTo<$target> for $source {
                const FIELD: &'static str = stringify!([<$target:camel Ref>]);
                fn linked(&self) -> Option<&$crate::common::TypedRef<$target>> {
                    self.[<$target:snake:lower _ref>].as_ref()
                }
            }
        }
    };
    ($source:ty => !$target:ty) => {
        ::paste::paste! {
            impl $crate::linked::LinkedTo<$target> for $source {
                const FIELD: &'static str = stringify!([<$target:camel Ref>]);
                fn linked(&self) -> Option<&$crate::common::TypedRef<$target>> {
                    Some(&self.[<$target:snake:lower _ref>])
                }
            }
        }
    };
}
