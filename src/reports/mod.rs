pub mod params;
pub mod types;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


/// Corresponds to the `ColumnTypeEnum` simpleType in the XSD.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ColumnTypeEnum {
    Account,
    Money,
    Rate,
    Customer,
    Vendor,
    Employee,
    ProductsAndService,
    Department,
    Class,
    StringValue,
}

/// Corresponds to the `RowTypeEnum` simpleType in the XSD.
#[derive(Debug, Serialize, Deserialize)]
pub enum RowTypeEnum {
    Section,
    Data,
}

/// Placeholder for the `ReportBasisEnum` reference (not defined in the provided XSD snippet).
/// Adjust these variants according to the actual definition if available.
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportBasisEnum {
    Cash,
    Accrual,
}

/// Represents a placeholder for an XSD type `NameValue`,
/// referred to in the snippet but not defined therein.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameValue {
    /// You can enhance this struct once the structure of `NameValue` is known.
    pub name: Option<String>,
    pub value: Option<String>,
}

/// Corresponds to the `Attribute` complexType in the XSD.
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// Corresponds to the `Attributes` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "Attribute", default)]
    pub attribute: Option<Vec<Attribute>>,
}

/// Corresponds to the `Columns` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Columns {
    #[serde(rename = "Column", default)]
    pub column: Option<Vec<Column>>,
}

/// Corresponds to the `Column` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Column {
    pub col_title: String,
    pub col_type: String,
    /// Repeats zero or more times, thus Option<Vec<...>>
    pub meta_data: Option<Vec<NameValue>>,
    /// Nested subcolumns
    pub columns: Option<Columns>,
}

/// Corresponds to the `ColData` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
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

/// Corresponds to the `Rows` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Rows {
    #[serde(rename = "Row", default)]
    pub row: Option<Vec<Row>>,
}

/// The `Row` complexType has a choice: either (Header, Rows, Summary) or (ColData repeated).
///
/// We capture this choice in an enum to represent mutually exclusive content.
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

/// Corresponds to the `Row` complexType in the XSD.
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

/// Corresponds to the `ReportHeader` complexType in the XSD.
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Report {
    pub header: Option<ReportHeader>,
    pub columns: Option<Columns>,
    pub rows: Option<Rows>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_report_ser_de() {
        let input = include_str!("../../test/data/report.json");
        let input_value = serde_json::from_str::<serde_json::Value>(input).unwrap();

        let _report: super::Report = serde_json::from_value(input_value.clone()).unwrap();
        // println!("{:#?}", _report);
        println!("{}", serde_json::to_string_pretty(&_report).unwrap());
        assert_eq!(input_value, serde_json::to_value(&_report).unwrap());
    }
}
