use crate::data::{Field, Row, Value};
use csv::{Reader, StringRecordsIter, Writer};

use std::collections::HashMap;
use std::fs::{File};

pub fn make_reader(path: &str) -> Reader<File> {
    Reader::from_path(path).unwrap()
}

pub fn make_writer(path: &str) -> Writer<File> {
    Writer::from_path(path).unwrap()
}

pub fn row_to_record(row: Row) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for field in row {
        let val = field.value;
        let v = match val {
            Value::Int(i) => i.to_string(),
            Value::Str(i) => i,
            Value::Null => String::from("")
        };

        output.push(v);
    }

    output
}

pub struct CsvRows<'c> {
    records: StringRecordsIter<'c, File>,
    schema: &'c HashMap<String, String>,
}

impl<'c> CsvRows<'c> {
    pub fn new(
        reader: &'c mut Reader<File>,
        schema: &'c HashMap<String, String>,
    ) -> CsvRows<'c> {
        let records = reader.records();
        CsvRows { records, schema }
    }
}

impl<'c> Iterator for CsvRows<'c> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.records.next();
        match res {
            Some(record) => {
                let rec = record.unwrap();
                let schema = self.schema;
                let row = make_row(rec, schema);
                Some(row)
            },
            None => None,
        }
    }
}

// --------------------------------------------------------
fn make_row<'r>(
    record: csv::StringRecord,
    schema: &'r HashMap<String, String>
) -> Row {
    let mut row: Row = Vec::new();
    let col_names = schema.keys();
    let col_types = schema.values();

    let val_name_typ = record.iter().zip(col_names).zip(col_types);
    for ((val, name), typ) in val_name_typ {
        let value = make_value(val, typ);
        let field = make_field(&name, value);
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

fn make_value(value: &str, given_type: &str) -> Value {
    match given_type {
        "str" => Value::Str(value.to_string()),
        "int" => {
            let value: i64 = value.parse().unwrap();
            Value::Int(value)
        },
        "null" => Value::Null,
        _ => Value::Str(value.to_string()),
    }
}
