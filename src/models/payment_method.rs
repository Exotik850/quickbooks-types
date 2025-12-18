use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[cfg(feature = "builder")]
use crate::error::QBTypeError;
use crate::{common::MetaData, QBCreatable, QBFullUpdatable, QBItem};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `PaymentMethod`
///
/// Represents a method of payment used in transactions.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/paymentmethod>
pub struct PaymentMethod {
    /// The unique ID of the payment method
    pub id: Option<String>,
    /// The unique sync token of the payment method, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the payment method
    pub name: Option<String>,
    /// Indicates if the payment method is active
    pub active: Option<bool>,
    /// Type of the payment method
    #[serde(rename = "Type")]
    pub payment_type: Option<PaymentMethodType>,
    /// Metadata about the payment method
    pub meta_data: Option<MetaData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Type of the payment method
pub enum PaymentMethodType {
    CreditCard,
    NonCreditCard,
}

impl QBCreatable for PaymentMethod {
    fn can_create(&self) -> bool {
        self.name.as_deref().is_some_and(|s| !s.trim().is_empty())
    }
}

impl QBFullUpdatable for PaymentMethod {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}
