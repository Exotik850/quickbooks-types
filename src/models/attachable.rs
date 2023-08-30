use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{QBCreatable, QBDeletable, QBError, QBFullUpdatable, QBItem, QBToRef};

use super::common::{CustomField, MetaData, NtRef};

/*
    Attachable Object
    https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable
*/

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(
    feature = "builder",
    builder(default, build_fn(error = "QBError"), setter(into, strip_option))
)]
pub struct Attachable {
    pub id: Option<String>,
    pub sync_token: Option<String>,
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    pub file_name: Option<String>,
    pub note: Option<String>,
    pub category: Option<AttachmentCategory>,
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    pub content_type: Option<String>,
    pub place_name: Option<String>,
    pub attachable_ref: Option<Vec<AttachableRef>>,
    pub long: Option<String>,
    pub tag: Option<String>,
    pub lat: Option<String>,
    pub file_access_uri: Option<String>,
    pub size: Option<f32>,
    pub thumbnail_file_access_uri: Option<String>,
    pub temp_download_uri: Option<String>,
}

#[must_use]
pub fn content_type_from_ext(ext: &str) -> &'static str {
    match ext {
        "ai" | "eps" => "application/postscript",
        "csv" => "text/csv",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "gif" => "image/gif",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpg",
        "png" => "image/png",
        "rtf" => "text/rtf",
        "txt" => "text/plain",
        "tif" => "image/tiff",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "pdf" => "application/pdf",
        "xls" => "application/vnd.ms-excel",
        "xml" => "text/xml",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        _ => panic!("Unsupported Format!"),
    }
}

#[cfg(feature = "builder")]
impl AttachableBuilder {
    pub fn file_name(&mut self, value: &dyn AsRef<Path>) -> Result<&mut Self, QBError> {
        let path = value.as_ref();

        self.file_name = Some(Some(
            path.file_name()
                .ok_or(QBError::ValidationError("Not a file!".into()))?
                .to_str()
                .ok_or(QBError::ValidationError(
                    "Could not turn file name into str".into(),
                ))?
                .into(),
        ));

        self.content_type = Some(Some(
            content_type_from_ext(
                path.extension()
                    .ok_or(QBError::ValidationError("No extension on file/dir".into()))?
                    .to_str()
                    .ok_or(QBError::ValidationError(
                        "Could not turn extension into string".into(),
                    ))?,
            )
            .into(),
        ));

        Ok(self)
    }
}

pub trait QBAttachable {
    fn can_upload(&self) -> bool;
    fn file_path(&self) -> Option<&String>;
}
impl QBAttachable for Attachable {
    fn can_upload(&self) -> bool {
        self.note.is_some() || self.file_name.is_some()
    }

    fn file_path(&self) -> Option<&String> {
        self.file_name.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum AttachmentCategory {
    ContactPhoto,
    Document,
    Image,
    Receipt,
    Signature,
    Sound,
    #[default]
    Other,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(default))]
pub struct AttachableRef {
    pub include_on_send: Option<bool>,
    pub line_info: Option<String>,
    pub no_ref_only: Option<bool>,
    pub custom_field: Option<Vec<CustomField>>,
    #[serde(rename = "type")]
    pub ref_type: Option<String>,
    pub inactive: Option<bool>,
    pub entity_ref: Option<NtRef>,
}

impl From<NtRef> for AttachableRef {
    fn from(value: NtRef) -> Self {
        AttachableRef {
            entity_ref: Some(value),
            ..Default::default()
        }
    }
}

impl<T: QBItem + QBToRef> From<T> for AttachableRef {
    fn from(value: T) -> Self {
        let value: NtRef = value.into();
        value.into()
    }
}

impl QBCreatable for Attachable {
    fn can_create(&self) -> bool {
        self.file_name.is_some() || self.note.is_some()
    }
}
impl QBDeletable for Attachable {}
impl QBFullUpdatable for Attachable {
    fn can_full_update(&self) -> bool {
        self.has_read() && self.can_create()
    }
}
impl QBToRef for Attachable {
    fn ref_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }
}
