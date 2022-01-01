use crate::data::{Field, Row, Value};

use serde_json::{Value as JsonValue, Map, json};

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Lines};

pub fn make_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
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
            Value::Null => JsonValue::Null,
        };
        output.insert(name.to_string(), val);
    }

    JsonValue::Object(output).to_string() + "\n"
}

pub struct JsonRows {
    lines: Lines<BufReader<File>>,
    schema: HashMap<String, String>,
}

impl JsonRows {
    pub fn new(
        reader: BufReader<File>,
        schema: HashMap<String, String>
    ) -> JsonRows {
        let lines = reader.lines();
        JsonRows { lines, schema }
    }
}

impl Iterator for JsonRows {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.lines.next();
        match res {
            Some(line) => {
                let l = line.unwrap();
                let row = make_row(l, &self.schema);
                Some(row)
            },
            None => None
        }
    }
}

// --------------------------------------------------------
fn make_row(
    line: String, schema: &HashMap<String, String>
) -> Row {
    let mut row: Row = Vec::new();
    let jobj: JsonValue = serde_json::from_str(&line).unwrap();

    for (col_name, col_type) in schema {
        let jfield = &jobj[&col_name];

        let value = make_value(&jfield, &col_type);
        let field = make_field(&col_name, value);
        row.push(field);
    }

    row
}

fn make_field<'f>(name: &'f str, value: Value) -> Field {
    Field {
        name: name.to_string(),
        value: value,
    }
}

fn make_value(value: &JsonValue, given_type: &str) -> Value {
    match given_type {
        "str" => {
            if let JsonValue::String(v) = value {
                Value::Str(v.to_string())
            } else {
                Value::Null
            }
        },
        "int" => {
            if let JsonValue::Number(v) = value {
                Value::Int(v.as_i64().unwrap())
            } else {
                Value::Null
            }
        },
        "null" => Value::Null,
        _ => Value::Null,
    }
}
