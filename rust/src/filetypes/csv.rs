use crate::data::{Field, Row, Value};
use csv::{Reader, StringRecordsIter, Writer};

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
    type Item = Row<'c>;

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

// --------------------------------------------------------
// ugliest possible thing
// pub struct CSV<'c> {
//     pub input_path: &'c str,
//     pub output_path: &'c str,
//     pub col_names: &'c Vec<String>,
//     pub col_types: &'c Vec<String>,
//     pub select: &'c Vec<String>,
// }

// impl<'c> CSV<'c> {
//     pub fn run(&self) -> Result<(), Box<dyn Error>> {
//         let mut rdr = csv::Reader::from_path(self.input_path).unwrap();
//         let mut wtr = csv::Writer::from_path(self.output_path).unwrap();

//         let records = rdr.records();

//         for result in records {
//             let str_row = result?;
//             let row = make_row(str_row, &self.col_names, &self.col_types);

//             let out_row = select(row, &self.select);
//             let out_str_row = deconstruct_row(out_row);
//             wtr.write_record(out_str_row)?;
//         }

//         wtr.flush()?;

//         Ok(())
//     }
// }

// --------------------------------------------------------
fn make_row<'r>(
    record: csv::StringRecord,
    col_names: &'r Vec<String>,
    col_types: &'r Vec<String>,
) -> Row<'r> {
    let mut row: Row = Vec::new();

    let val_name_typ = record.iter().zip(col_names.iter()).zip(col_types.iter());
    for ((val, name), typ) in val_name_typ {
        let value = make_value(val, typ);
        let field = make_field(&name, value);
        row.push(field);
    }

    row
}

// fn deconstruct_row(row: Row) -> Vec<String> {
//     let mut output: Vec<String> = Vec::new();

//     for field in row {
//         let val = field.value;
//         let v = match val {
//             Value::Int(i) => i.to_string(),
//             Value::Str(i) => i,
//         };

//         output.push(v);
//     }

//     output
// }

fn make_field<'f>(name: &'f str, value: Value) -> Field<'f> {
    Field {
        name: name,
        value: value,
    }
}

fn make_value(value: &str, given_type: &str) -> Value {
    match given_type {
        "str" => Value::Str(value.to_string()),
        "int" => {
            let value: i64 = value.parse().unwrap();
            Value::Int(value)
        }
        _ => Value::Str(value.to_string()),
    }
}
