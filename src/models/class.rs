use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{MetaData, TypedRef},
    impl_linked, QBCreatable, QBFullUpdatable,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `Class`
///
/// Represents a classification or category that can be assigned to transactions and other entities in `QuickBooks` Online.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/class>
pub struct Class {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    pub name: Option<String>,
    pub parent_ref: Option<TypedRef<Class>>,
    pub sub_class: Option<bool>,
    pub active: Option<bool>,
    pub meta_data: Option<MetaData>,
    pub fully_qualified_name: Option<String>,
}

impl_linked!(Class as parent_ref => Class);

impl QBCreatable for Class {
    fn can_create(&self) -> bool {
        self.name.as_deref().is_some_and(|s| !s.trim().is_empty())
            && !self.sub_class.unwrap_or(false)
            || self.parent_ref.is_some()
    }
}
impl QBFullUpdatable for Class {
    fn can_full_update(&self) -> bool {
        self.id.is_some() && self.sync_token.is_some() && self.can_create()
    }
}
