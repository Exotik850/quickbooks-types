use super::Column;
use crate::reports::{ColData, ColumnTypeEnum, Row, RowContent, Rows};
use polars::prelude::*;
use std::num::ParseFloatError;

#[derive(thiserror::Error, Debug)]
pub enum QBPolarsError {
    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),
    #[error("No columns found in report")]
    NoColumnsFound,
    #[error("Invalid shape of rows in report")]
    InvalidRowShape,
}

impl super::Report {
    // Function to convert Report into Polars DataFrame
    pub fn into_dataframe(self) -> Result<DataFrame, QBPolarsError> {
        // 1) Flatten and collect column definitions (name + type)
        let cols = self.columns.and_then(|c| c.column);
        let leaf_rows = self.rows.map(collect_leaf_rows);

        let (cols, leaf_rows) = match (cols, leaf_rows) {
            // There is no columns and no rows, so we return an empty DataFrame
            (None, None) => return Ok(DataFrame::empty()),
            // We cant construct a DataFrame without columns, so we return an error
            (None, Some(_)) => {
                return Err(QBPolarsError::NoColumnsFound);
            }
            // We have columns, but no rows, so we return an empty DataFrame
            (Some(c), None) => (c, Vec::new()),
            (Some(c), Some(lr)) => (c, lr),
        };

        let mut col_defs = Vec::new();
        flatten_columns(cols, None, &mut col_defs);

        // 2) Collect all leaf rows (those carrying ColData)

        if leaf_rows.is_empty() {
            // If we have column definitions, we create an empty DataFrame with those columns
            if col_defs.is_empty() {
                return Ok(DataFrame::empty());
            };
            let series = col_defs
                .into_iter()
                .map(|(col_name, _)| Series::new_empty(col_name.into(), &DataType::String))
                .map(polars::prelude::Column::from);
            return Ok(DataFrame::from_iter(series));
        }

        // Check that the rows have the same number of columns as defined
        if leaf_rows.iter().any(|row| row.len() != col_defs.len()) {
            return Err(QBPolarsError::InvalidRowShape);
        }

        let series = col_defs
            .into_iter()
            .enumerate()
            .map(|(idx, (col_name, col_type))| {
                create_series_for_col(col_name, col_type, idx, &leaf_rows)
            });

        let columns = series.map(polars::prelude::Column::from).collect();

        // 4) Construct DataFrame
        DataFrame::new(columns).map_err(Into::into)
    }
}

fn create_series_for_col(
    col_name: String,
    col_type: ColumnTypeEnum,
    idx: usize,
    rows: &[Vec<ColData>],
) -> Series {
    match col_type {
        ColumnTypeEnum::Money | ColumnTypeEnum::Rate => {
            create_numeric_or_string_series(col_name, idx, rows)
        }
        _ => create_string_series(col_name, idx, rows),
    }
}

fn create_numeric_or_string_series(col_name: String, idx: usize, rows: &[Vec<ColData>]) -> Series {
    let values: Vec<_> = rows.iter().map(|row| row[idx].value.as_deref()).collect();
    if let Ok(numeric_values) = try_parse_as_numeric(values) {
        Float64Chunked::from_iter_options(col_name.into(), numeric_values.into_iter()).into_series()
    } else {
        create_string_series(col_name, idx, rows)
    }
}

fn try_parse_as_numeric(values: Vec<Option<&str>>) -> Result<Vec<Option<f64>>, ParseFloatError> {
    values
        .into_iter()
        .map(|value| {
            value
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_matches(|c: char| c.is_whitespace() || c == '%'))
                .map(str::parse::<f64>)
                .transpose()
        })
        .collect()
}

fn create_string_series(col_name: String, idx: usize, rows: &[Vec<ColData>]) -> Series {
    StringChunked::from_iter_options(
        col_name.into(),
        rows.iter().map(|row| row[idx].value.as_deref()),
    )
    .into_series()
}

/// Recursively flatten Columns into (column_name, column_type) tuples.
/// If `prefix` is Some, it's the parent column's title, joined by " - ".
fn flatten_columns(
    cols: Vec<Column>,
    prefix: Option<&str>,
    out: &mut Vec<(String, ColumnTypeEnum)>,
) {
    for col in cols {
        let name = col
            .col_title
            .is_empty()
            .then(|| col.col_type.as_str())
            .unwrap_or_else(|| col.col_title.as_str());
        let title = if let Some(p) = prefix {
            format!("{} - {}", p, name)
        } else {
            name.to_string()
        };
        if let Some(sub) = col.columns.and_then(|s| s.column) {
            flatten_columns(sub, Some(&title), out);
            continue;
        }
        // leaf column
        out.push((title, col.col_type));
    }
}

/// Recursively collect leaf rows carrying `ColData`.
fn collect_leaf_rows(rows: Rows) -> Vec<Vec<ColData>> {
    let mut out = Vec::new();
    if let Some(inner) = rows.row {
        for row in inner {
            collect_from_row(row, &mut out);
        }
    }
    out
}

fn collect_from_row(row: Row, out: &mut Vec<Vec<ColData>>) {
    match row.content {
        RowContent::Coldata { col_data } => {
            out.push(col_data);
        }
        RowContent::HeaderRowsSummary { rows, .. } => {
            if let Some(subrows) = rows {
                if let Some(inner) = subrows.row {
                    for r in inner {
                        collect_from_row(r, out);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_report_to_dataframe() {
        let input = include_str!("../../test/data/report2.json");
        let report: super::super::Report = serde_json::from_str(input).unwrap();
        let mut df_result = report.into_dataframe().unwrap();

        // Check if the DataFrame is not empty
        assert!(!df_result.is_empty());

        // Print the DataFrame for debugging
        println!("{:?}", df_result);

        // let path = "./report.csv";
        // let file = std::fs::OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open(path)
        //     .unwrap();
        // let mut writer = polars::io::csv::write::CsvWriter::new(file);
        // writer.finish(&mut df_result).unwrap();
    }
}
