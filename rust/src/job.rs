use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub input_path: String,
    pub structure: CsvFile,
    pub output_path: String,
    pub select: Option<Select>,
    pub filter: Option<Vec<Filter>>,
}

#[derive(Serialize, Deserialize)]
pub struct CsvFile {
    pub col_names: Vec<String>,
    pub col_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonLinesFile {
    pub schema: HashMap<String, String>
}

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
