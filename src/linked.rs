use crate::common::TypedRef;
use crate::models::*;

pub trait Linked<O> {
    fn linked_ref(&self) -> Option<&TypedRef<O>>;
}

impl Linked<Customer> for Invoice {
    fn linked_ref(&self) -> Option<&TypedRef<Customer>> {
        self.customer_ref.as_ref()
    }
}
