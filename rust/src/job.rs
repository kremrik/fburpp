use crate::{csv, json};
use crate::data::{filter, select};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Select {
    pub fields: Vec<String>
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub comparator: String,
    pub value: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum FileType {
    csv { col_names: Vec<String>, col_types: Vec<String> },
    // json { schema: HashMap<String, String> }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Job {
    pub input_path: String,
    pub input_type: String,
    pub schema: FileType,
    pub output_path: String,
    pub output_type: String,
    pub select: Option<Select>,
    pub filter: Option<Vec<Filter>>,
}

impl Job {
    pub fn execute(&mut self) {
        let mut handler = match self.input_type.as_str() {
            "csv" => csv::make_file_handler(&self.input_path),
            _ => panic!("CRAP!")
        };

        let rows = match &self.schema {
            FileType::csv { col_names, col_types } => {
                csv::CsvRows::new(&mut handler, &col_names, &col_types)
            },
        };

        let mut writer = match self.output_type.as_str() {
            "csv" => {
                csv::CsvWriter::new(&self.output_path)
            },
            _ => panic!("CRAP!")
        };

        let sel = &self.select;

        for row in rows {
            let sel_row = select(row, &sel);
            writer.write(sel_row).unwrap();
        }

    }
}
