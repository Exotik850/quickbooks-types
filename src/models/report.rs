use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Corresponds to the `DateMacro` simpleType in the XSD.
#[derive(Debug, Serialize, Deserialize)]
pub enum DateMacro {
    #[serde(rename = "All")]
    All,
    #[serde(rename = "Today")]
    Today,
    #[serde(rename = "This Week")]
    ThisWeek,
    #[serde(rename = "This Week-to-date")]
    ThisWeekToDate,
    #[serde(rename = "This Month")]
    ThisMonth,
    #[serde(rename = "This Month-to-date")]
    ThisMonthToDate,
    #[serde(rename = "This Fiscal Quarter")]
    ThisFiscalQuarter,
    #[serde(rename = "This Fiscal Quarter-to-date")]
    ThisFiscalQuarterToDate,
    #[serde(rename = "This Fiscal Year")]
    ThisFiscalYear,
    #[serde(rename = "This Fiscal Year-to-date")]
    ThisFiscalYearToDate,
    #[serde(rename = "This Calendar Quarter")]
    ThisCalendarQuarter,
    #[serde(rename = "This Calendar Quarter-to-date")]
    ThisCalendarQuarterToDate,
    #[serde(rename = "This Calendar Year")]
    ThisCalendarYear,
    #[serde(rename = "This Calendar Year-to-date")]
    ThisCalendarYearToDate,
    #[serde(rename = "Yesterday")]
    Yesterday,
    #[serde(rename = "Last Week")]
    LastWeek,
    #[serde(rename = "Last Week-to-date")]
    LastWeekToDate,
    #[serde(rename = "Last Month")]
    LastMonth,
    #[serde(rename = "Last Month-to-date")]
    LastMonthToDate,
    #[serde(rename = "Last Fiscal Quarter")]
    LastFiscalQuarter,
    #[serde(rename = "Last Fiscal Quarter-to-date")]
    LastFiscalQuarterToDate,
    #[serde(rename = "Last Fiscal Year")]
    LastFiscalYear,
    #[serde(rename = "Last Fiscal Year-to-date")]
    LastFiscalYearToDate,
    #[serde(rename = "Last Calendar Quarter")]
    LastCalendarQuarter,
    #[serde(rename = "Last Calendar Quarter-to-date")]
    LastCalendarQuarterToDate,
    #[serde(rename = "Last Calendar Year")]
    LastCalendarYear,
    #[serde(rename = "Last Calendar Year-to-date")]
    LastCalendarYearToDate,
    #[serde(rename = "Next Week")]
    NextWeek,
    #[serde(rename = "Next 4 Weeks")]
    Next4Weeks,
    #[serde(rename = "Next Month")]
    NextMonth,
    #[serde(rename = "Next Fiscal Quarter")]
    NextFiscalQuarter,
    #[serde(rename = "Next Fiscal Year")]
    NextFiscalYear,
    #[serde(rename = "Next Calendar Quarter")]
    NextCalendarQuarter,
    #[serde(rename = "Next Calendar Year")]
    NextCalendarYear,
}

/// Corresponds to the `SummarizeColumnsByEnum` simpleType in the XSD.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SummarizeColumnsByEnum {
    Total,
    Year,
    Quarter,
    FiscalYear,
    FiscalQuarter,
    Month,
    Week,
    Days,
    Customers,
    Vendors,
    Employees,
    Departments,
    Classes,
    ProductsAndServices,
}

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

/// Corresponds to the `Rows` complexType in the XSD.
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
#[serde(rename_all = "PascalCase", untagged)]
pub enum RowContent {
    /// Contains a list of `ColData` elements.
    ColdataVec {
        #[serde(rename = "ColData")]
        col_data: Vec<ColData>,
    },
    /// Contains `Header`, `Rows`, and `Summary` together.
    HeaderRowsSummary {
        header: Option<Vec<ColData>>,
        summary: Option<Vec<ColData>>,
        rows: Option<Rows>,
    },
}

/// Corresponds to the `Row` complexType in the XSD.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "id", default)]
    pub id: Option<String>,
    #[serde(rename = "parentId", default)]
    pub parent_id: Option<String>,
    #[serde(flatten)]
    pub content: RowContent,

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
    pub time: Option<String>,
    pub report_name: Option<String>,
    pub date_macro: Option<String>,
    pub report_basis: Option<ReportBasisEnum>,
    pub start_period: Option<String>,
    pub end_period: Option<String>,
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
