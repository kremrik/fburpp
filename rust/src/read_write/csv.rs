use crate::data::{select, Field, Row, Value};
use csv;

use std::error::Error;

// ugliest possible thing
pub struct CSV<'c> {
    pub input_path: &'c str,
    pub output_path: &'c str,
    pub col_names: Vec<&'c str>,
    pub col_types: Vec<&'c str>,
    pub select: Vec<&'c str>,
}

impl<'c> CSV<'c> {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(self.input_path).unwrap();
        let mut wtr = csv::Writer::from_path(self.output_path).unwrap();

        let records = rdr.records();

        for result in records {
            let str_row = result?;
            let row = make_row(str_row, &self.col_names, &self.col_types);

            let out_row = select(row, &self.select);
            let out_str_row = deconstruct_row(out_row);
            wtr.write_record(out_str_row)?;
        }

        wtr.flush()?;

        Ok(())
    }
}

// --------------------------------------------------------
fn make_row<'r>(
    record: csv::StringRecord,
    col_names: &'r Vec<&str>,
    col_types: &'r Vec<&str>,
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

fn deconstruct_row(row: Row) -> Vec<String> {
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
