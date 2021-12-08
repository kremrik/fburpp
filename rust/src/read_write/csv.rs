use csv;

use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct Field<'f> {
    name: &'f str,
    value: Value,
}

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i64),
}

type Row<'r> = Vec<Field<'r>>;
// type Frame<'f> = Vec<Row<'f>>;

pub struct RowIterator<'c> {
    pub col_names: &'c Vec<&'c str>,
    pub col_types: &'c Vec<&'c str>,
    pub records: csv::StringRecordsIter<'c, File>
}

impl<'c> Iterator for RowIterator<'c> {
    type Item = Row<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        let record = self.records.next();
        
        match record {
            Some(r) => {
                let col_names = self.col_names;
                let col_types = self.col_types;
                let row = make_row(r.unwrap(), col_names, col_types);
                return Some(row)
            },
            None => None
        }
    }
}

pub struct RowWriter {
    pub writer: csv::Writer<File>
}

impl RowWriter {
    pub fn write(
        &mut self, row: Row
    ) -> Result<(), Box<dyn Error>> {
        let d_row = deconstruct_row(row);
        self.writer.write_record(d_row)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.flush()?;
        Ok(())
    }
}


// --------------------------------------------------------
fn deconstruct_row(row: Row) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for field in row {
        let val = field.value;
        let v = match val {
            Value::Int(i) => i.to_string(),
            Value::Str(i) => i
        };

        output.push(v);
    }

    output
}

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
