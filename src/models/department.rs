use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{MetaData, TypedRef},
    impl_linked, QBCreatable, QBFullUpdatable, QBItem,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// Department
///
/// Represents a department within a company, used for categorizing transactions and tracking financial data.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/department>
pub struct Department {
    /// The unique ID of the department
    pub id: Option<String>,
    /// The unique sync token of the department, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the department
    pub name: Option<String>,
    /// Indicates if the department is active
    pub active: Option<bool>,
    /// Metadata about the department
    pub meta_data: Option<MetaData>,
    /// Indicates if the department is a sub-department
    pub sub_department: Option<bool>,
    /// Reference to the parent department if this is a sub-department
    pub parent_ref: Option<TypedRef<Department>>,
    /// Fully qualified name of the department
    pub fully_qualified_name: Option<String>,
}

impl QBCreatable for Department {
    fn can_create(&self) -> bool {
        self.name.is_some() && (!self.sub_department.unwrap_or(false) || self.parent_ref.is_some())
    }
}
impl QBFullUpdatable for Department {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}

impl_linked!(Department as parent_ref => Department);
