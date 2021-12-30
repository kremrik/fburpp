use crate::data::{Field, Row, Value};

use serde_json::{Result, Value as JsonValue, Map, Number, json};

use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};

pub fn make_reader(path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        println!("{:?}", line);
    }
}

pub fn make_writer(path: &str) -> BufWriter<File> {
    let file = File::create(path).unwrap();
    BufWriter::new(file)
}

pub fn row_to_object(row: Row) -> String {
    let mut output = Map::new();

    for field in row {
        let name = field.name;
        let val = match field.value {
            Value::Str(s) => json!(s),
            Value::Int(i) => json!(i),
        };
        output.insert(name.to_string(), val);
    }

    JsonValue::Object(output).to_string() + "\n"
}
