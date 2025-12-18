use crate::common::TypedRef;

// we can represent the child -> parent relationship for now,
// parent -> children requires querying the API, which is out of scope here

/// Trait for "this object has a single linked object of type `Target`".
///
/// Returns an optional reference to the linked object.
pub trait LinkedTo<Target> {
    fn linked(&self) -> Option<&TypedRef<Target>>;
}

#[macro_export]
macro_rules! impl_linked {
    ($source:ty as $field:tt => $target:ty) => {
        ::paste::paste! {
            impl $crate::linked::LinkedTo<$target> for $source {
                fn linked(&self) -> Option<&TypedRef<$target>> {
                    self.$field.as_ref()
                }
            }
        }
    };
    ($source:ty => $target:ty) => {
        ::paste::paste! {
            impl $crate::linked::LinkedTo<$target> for $source {
                fn linked(&self) -> Option<&TypedRef<$target>> {
                    self.[<$target:snake:lower _ref>].as_ref()
                }
            }
        }
    };
}
