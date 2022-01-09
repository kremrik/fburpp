use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub input_path: String,
    pub file_type: String,
    pub schema: HashMap<String, String>,
    pub output_path: String,
    pub select: Option<Select>,
    pub filter: Option<Vec<Filter>>,
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
