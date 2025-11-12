#[cfg(feature = "builder")]
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common::{CustomField, MetaData, NtRef};
use crate::{QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBToRef, QBTypeError};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]

/// Attachable
///
/// Represents a file attachment or note that can be linked to other QuickBooks entities (for example: Invoice, Bill, Customer).
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/attachable>
pub struct Attachable {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    #[serde(skip_serializing)]
    pub meta_data: Option<MetaData>,
    /// File name of the attachment
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    pub file_name: Option<String>,
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    #[serde(skip)]
    pub file_path: Option<PathBuf>,
    /// Private note for the attachment
    pub note: Option<String>,
    /// Category of the attachment
    pub category: Option<AttachmentCategory>,
    /// Content type of the attachment
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    pub content_type: Option<String>,
    pub place_name: Option<String>,
    /// References to the transaction object to which this attachable file is to be linked
    #[cfg_attr(feature = "builder", builder(setter(strip_option)))]
    pub attachable_ref: Option<Vec<AttachableRef>>,
    /// Longitude of the place where the attachment was taken
    pub long: Option<String>,
    /// Tag for the attachment
    pub tag: Option<String>,
    /// Latitude of the place where the attachment was taken
    pub lat: Option<String>,
    /// URI for accessing the file
    pub file_access_uri: Option<String>,
    /// Size of the file in bytes
    pub size: Option<f64>,
    /// URI for accessing the thumbnail of the file
    pub thumbnail_file_access_uri: Option<String>,
    /// Temporary download URI for the file
    pub temp_download_uri: Option<String>,
}

/// Derives the content type from a file extension
pub fn content_type_from_ext(ext: &str) -> Option<&'static str> {
    let out = match ext {
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
        _ => return None,
    };
    Some(out)
}

#[cfg(feature = "builder")]
impl AttachableBuilder {
    /// Sets the file_path (for_upload) and derives the file_name and content_type from it
    pub fn file(&mut self, path: &impl AsRef<Path>) -> Result<&mut Self, QBTypeError> {
        let path = path.as_ref();

        self.file_path = Some(Some(path.to_path_buf()));

        self.file_name = Some(Some(
            path.file_name()
                .ok_or(QBTypeError::ValidationError(
                    "No file name on file path".into(),
                ))?
                .to_str()
                .ok_or(QBTypeError::ValidationError(
                    "Could not turn file name into string".into(),
                ))?
                .to_owned(),
        ));

        self.content_type = Some(
            content_type_from_ext(
                path.extension()
                    .ok_or(QBTypeError::ValidationError(
                        "No extension on file/dir".into(),
                    ))?
                    .to_string_lossy()
                    .as_ref(),
            )
            .map(|f| f.to_owned()),
        );

        Ok(self)
    }
}

/// Trait for all entities that can be attached as files/notes
pub trait QBAttachable {
    fn can_upload(&self) -> Result<(), QBTypeError>;
    fn file_path(&self) -> Option<&String>;
}
impl QBAttachable for Attachable {
    fn can_upload(&self) -> Result<(), QBTypeError> {
        if self.note.is_none() && self.file_name.is_none() {
            return Err(QBTypeError::MissingField("note"));
        }
        if self.file_name.is_none() {
            return Err(QBTypeError::MissingField("file_name"));
        }
        Ok(())
    }

    fn file_path(&self) -> Option<&String> {
        self.file_name.as_ref()
    }
}

/// AttachmentCategory
///
/// Enumerates the category of an attachment (for example: Document, Image, Receipt).
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

/// AttachableRef
///
/// A reference that links an attachment to a target QuickBooks entity (such as an Invoice line or a Bill).
///
/// Most callers will construct this via `QBToAttachableRef::to_attach_ref()` on an entity that implements `QBToRef`.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(feature = "builder", derive(Builder), builder(default))]
pub struct AttachableRef {
    /// Indicates if the entity should be included on send
    pub include_on_send: Option<bool>,
    /// Line information for the entity
    pub line_info: Option<String>,
    /// Indicates if the entity is a reference only
    pub no_ref_only: Option<bool>,
    /// Custom fields for the entity
    pub custom_field: Option<Vec<CustomField>>,
    /// Type of the entity
    #[serde(rename = "type")]
    pub ref_type: Option<String>,
    /// Indicates if the entity is inactive
    pub inactive: Option<bool>,
    /// The unique ID of the entity
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

/// Trait for entities that can be converted to a reference for an attachment.
pub trait QBToAttachableRef: QBToRef {
    fn to_attach_ref(&self) -> Result<AttachableRef, QBTypeError> {
        let value = self.to_ref()?;
        Ok(value.into())
    }
}

impl<T: QBToRef> QBToAttachableRef for T {}

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
