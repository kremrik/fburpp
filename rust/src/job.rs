use crate::{csv, json};
use crate::data::{filter, select};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;


#[derive(Serialize, Deserialize)]
pub struct Select {
    pub fields: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub comparator: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub input_path: String,
    pub file_type: String,
    pub schema: HashMap<String, String>,
    pub output_path: String,
    pub select: Option<Select>,
    pub filter: Option<Vec<Filter>>,
}

impl Job {
    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        if self.file_type == "csv" {
            let mut reader = csv::make_reader(&self.input_path);
            let mut writer = csv::make_writer(&self.output_path);
            let csvrows = csv::CsvRows::new(&mut reader, &self.schema);

            let sel = &self.select;

            for row in csvrows {
                let sel_row = select(row, &sel);
                let record = csv::row_to_record(sel_row);
                writer.write_record(record)?;
            }

        Ok(())

        } else if self.file_type == "json" {
            Ok(())
        } else {
            Ok(())
        }
    }
}
