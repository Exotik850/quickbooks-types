use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Corresponds to the `ColumnTypeEnum`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ColumnTypeEnum {
    /// Represents an account type, such as a bank or credit card account.
    Account,
    /// Monetary values.
    Money,
    /// This column type is used for rates (e.g., hourly rates).
    Rate,
    /// Customer ID or name.
    Customer,
    /// Vendor ID or name.
    Vendor,
    /// Employee ID or name.
    Employee,
    /// Item ID or name.
    ProductsAndService,
    /// Department ID or name.
    Department,
    /// Class ID or name.
    Class,
    /// String values.
    StringValue,
}

impl ColumnTypeEnum {
    /// Returns the string representation of the column type.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            ColumnTypeEnum::Account => "Account",
            ColumnTypeEnum::Money => "Money",
            ColumnTypeEnum::Rate => "Rate",
            ColumnTypeEnum::Customer => "Customer",
            ColumnTypeEnum::Vendor => "Vendor",
            ColumnTypeEnum::Employee => "Employee",
            ColumnTypeEnum::ProductsAndService => "ProductsAndService",
            ColumnTypeEnum::Department => "Department",
            ColumnTypeEnum::Class => "Class",
            ColumnTypeEnum::StringValue => "StringValue",
        }
    }
}

impl std::fmt::Display for ColumnTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Tells whether the row is a section header or data row.
#[derive(Debug, Serialize, Deserialize)]
pub enum RowTypeEnum {
    Section,
    Data,
}

/// Indicates whether the report is based on cash or accrual accounting.
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportBasisEnum {
    Cash,
    Accrual,
}

/// Corresponds to the `NameValue`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameValue {
    pub name: Option<String>,
    pub value: Option<String>,
}

/// Corresponds to the `Attribute`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// Corresponds to the `Attributes`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attributes {
    #[serde(rename = "Attribute", default)]
    pub attribute: Option<Vec<Attribute>>,
}

/// Corresponds to the `Columns`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Columns {
    #[serde(rename = "Column", default)]
    pub column: Option<Vec<Column>>,
}

/// Corresponds to the `Column`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Column {
    pub col_title: String,
    pub col_type: ColumnTypeEnum,
    /// Repeats zero or more times, thus Option<Vec<...>>
    pub meta_data: Option<Vec<NameValue>>,
    /// Nested subcolumns
    pub columns: Option<Columns>,
}

/// Corresponds to the `ColData`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColData {
    /// Nested Attributes element
    #[serde(rename = "Attributes", default)]
    pub attributes: Option<Attributes>,
    pub value: Option<String>,
    pub id: Option<String>,
    pub href: Option<String>,
}

/// Represents a collection of `ColData` elements.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColDataCollection {
    pub col_data: Option<Vec<ColData>>,
}

/// Corresponds to the `Rows`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Rows {
    #[serde(rename = "Row", default)]
    pub row: Option<Vec<Row>>,
}

/// The `Row` complexType has a choice: either (Header, Rows, Summary) or (`ColData` repeated).
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RowContent {
    /// Contains a list of `ColData` elements.
    Coldata {
        #[serde(rename = "ColData")]
        col_data: Vec<ColData>,
    },
    /// Contains `Header`, `Rows`, and `Summary` together.
    HeaderRowsSummary {
        #[serde(rename = "Header")]
        header: Option<ColDataCollection>,
        #[serde(rename = "Summary")]
        summary: Option<ColDataCollection>,
        #[serde(rename = "Rows")]
        rows: Option<Rows>,
    },
}

/// Corresponds to the `Row`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    #[serde(flatten)]
    pub content: RowContent,
    #[serde(rename = "id", default)]
    pub id: Option<String>,
    #[serde(rename = "parentId", default)]
    pub parent_id: Option<String>,

    /// Row attributes
    #[serde(rename = "type")]
    pub row_type: Option<RowTypeEnum>, // Not needed as it's part of content choice
    pub group: Option<String>,
}

/// Corresponds to the `ReportHeader`.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportHeader {
    pub time: Option<DateTime<FixedOffset>>,
    pub report_name: Option<String>,
    pub date_macro: Option<String>,
    pub report_basis: Option<ReportBasisEnum>,
    pub start_period: Option<NaiveDate>,
    pub end_period: Option<NaiveDate>,
    pub summarize_columns_by: Option<String>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub vendor: Option<String>,
    pub employee: Option<String>,
    pub item: Option<String>,
    pub class_attr: Option<String>,
    pub department: Option<String>,
    pub option: Option<Vec<NameValue>>,
}

/// Report structure containing header, columns, and rows.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Report {
    pub header: Option<ReportHeader>,
    pub columns: Option<Columns>,
    pub rows: Option<Rows>,
}
