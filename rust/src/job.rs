use crate::{csv, data, json};
use crate::data::{filter, select};

use ::csv::Reader;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fs::{File};
use std::io::{BufReader, Read};


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
    json { schema: HashMap<String, String> }
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

enum JobReader {
    Csv(Reader<File>),
    Json(BufReader<File>)
}

enum JobWriter {
    Csv(csv::CsvWriter),
    Json(json::JsonWriter)
}

impl Job {
    pub fn execute(&mut self) {

        let mut reader = match self.input_type.as_str() {
            "csv" => {
                JobReader::Csv(csv::make_reader(&self.input_path))
            },
            "json" => JobReader::Json(json::make_reader(&self.input_path)),
            _ => panic!("CRAP!")
        };

        let rows: Box<dyn Iterator<Item=data::Row>> = match &self.schema {
            FileType::csv { col_names, col_types } => {
                if let JobReader::Csv(ref mut r) = reader {
                    Box::new(csv::CsvRows::new(r, &col_names, &col_types))
                } else {
                    panic!("CRAP!");
                }
            },
            FileType::json { schema } => {
                if let JobReader::Json(r) = reader {
                    Box::new(json::JsonRows::new(r, schema))
                } else {
                    panic!("CRAP!")
                }
            }
        };

        let mut writer: Box<dyn data::Writer> = match self.output_type.as_str() {
            "csv" => Box::new(csv::CsvWriter::new(&self.output_path)),
            "json" => Box::new(json::JsonWriter::new(&self.output_path)),
            _ => panic!("CRAP!")
        };

        let sel = &self.select;

        for row in rows {
            let sel_row = select(row, &sel);
            writer.write(sel_row);
        }

    }
}
