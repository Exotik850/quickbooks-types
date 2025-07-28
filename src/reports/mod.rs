mod models;
pub mod params;
pub mod types;
pub use models::*;

#[cfg(feature = "polars")]
mod polars;
#[cfg(feature = "polars")]
pub use polars::{PolarsReport, QBPolarsError};

impl Report {
    pub fn name(&self) -> Option<&str> {
        self.header.as_ref().and_then(|h| h.report_name.as_deref())
    }

    pub fn col_data(&self, column: &str) -> Option<impl Iterator<Item = &ColData>> {
        let index = self
            .columns
            .as_ref()?
            .column
            .as_ref()?
            .iter()
            .position(|c| c.col_title == column)?;
        self.row_data().map(|rows| {
            rows.filter_map(move |row| {
                if let Some(col_data) = row.get(index) {
                    Some(col_data)
                } else {
                    None
                }
            })
        })
    }

    pub fn column_names(&self) -> Option<impl Iterator<Item = &str>> {
        self.columns
            .as_ref()?
            .column
            .as_ref()
            .map(|cols| cols.iter().map(|c| c.col_title.as_str()))
    }

    pub fn row_data(&self) -> Option<impl Iterator<Item = &[ColData]>> {
        self.rows.as_ref()?.row.as_ref().map(|rows| {
            rows.iter().filter_map(|row| {
                if let RowContent::Coldata { col_data } = &row.content {
                    Some(col_data.as_slice())
                } else {
                    None
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_report_ser_de() {
        let input = include_str!("../../test/data/report1.json");
        let input_value = serde_json::from_str::<serde_json::Value>(input).unwrap();

        let _report: super::Report = serde_json::from_value(input_value.clone()).unwrap();
        // println!("{:#?}", _report);
        println!("{}", serde_json::to_string_pretty(&_report).unwrap());
        assert_eq!(input_value, serde_json::to_value(&_report).unwrap());
    }

    #[test]
    fn test_report_column_names() {
        let input = include_str!("../../test/data/report1.json");
        let report: super::Report = serde_json::from_str(input).unwrap();
        let column_names: Vec<_> = report.column_names().unwrap().collect();
        assert!(!column_names.is_empty());
        println!("{:?}", column_names);
    }
}
