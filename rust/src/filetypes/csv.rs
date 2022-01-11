use crate::data::{Field, Row, Value, Writer as RowWriter};
use csv::{Reader, StringRecordsIter, Writer};

use std::error::Error;
use std::fs::{File};


pub fn make_reader(path: &str) -> Reader<File> {
    Reader::from_path(path).unwrap()
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
    col_names: &'c Vec<String>,
    col_types: &'c Vec<String>,
}

impl<'c> CsvRows<'c> {
    pub fn new(
        reader: &'c mut Reader<File>,
        col_names: &'c Vec<String>,
        col_types: &'c Vec<String>,
    ) -> CsvRows<'c> {
        let records = reader.records();
        CsvRows { records, col_names, col_types }
    }
}

impl<'c> Iterator for CsvRows<'c> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.records.next();
        match res {
            Some(record) => {
                let rec = record.unwrap();
                let names = self.col_names;
                let types = self.col_types;
                let row = make_row(rec, names, types);
                Some(row)
            },
            None => None,
        }
    }
}

pub struct CsvWriter {
    writer: Writer<File>
}

impl CsvWriter {
    pub fn new(path: &str) -> CsvWriter {
        let writer = Writer::from_path(path).unwrap();
        return CsvWriter { writer }
    }
}

impl RowWriter for CsvWriter {
    fn write(&mut self, row: Row) {
        let output = row_to_record(row);
        self.writer.write_record(output).unwrap();
    }
}

// --------------------------------------------------------
fn make_row<'r>(
    record: csv::StringRecord,
    col_names: &'r Vec<String>,
    col_types: &'r Vec<String>,
) -> Row {
    let mut row: Row = Vec::new();

    let val_name_typ = record.iter().zip(col_names.iter()).zip(col_types.iter());
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
