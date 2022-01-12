use crate::data::{Field, Row, Value, Writer as RowWriter};

use serde_json::{Value as JsonValue, Map, json};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Lines};

pub fn make_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

pub struct JsonRows<'j> {
    lines: Lines<BufReader<File>>,
    schema: &'j HashMap<String, String>,
}

impl<'j> JsonRows<'j> {
    pub fn new(
        buffer: BufReader<File>,
        schema: &HashMap<String, String>
    ) -> JsonRows {
        let lines = buffer.lines();
        JsonRows { lines, schema }
    }
}

impl<'j> Iterator for JsonRows<'j> {
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

pub struct JsonWriter {
    writer: BufWriter<File>
}

impl JsonWriter {
    pub fn new(path: &str) -> JsonWriter {
        let writer = BufWriter::new(File::create(path).unwrap());
        return JsonWriter { writer }
    }
}

impl RowWriter for JsonWriter {
    fn write(&mut self, row: Row) {
        let output = row_to_object(row);
        let outputbytes = output.as_bytes();
        self.writer.write_all(outputbytes).unwrap();
    }
}

// --------------------------------------------------------
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
