use csv;

use std::error::Error;


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
type Frame<'f> = Vec<Row<'f>>;

pub fn reader<'r>(
    path: &str,
    col_names: &'r Vec<&str>,
    col_types: &'r Vec<&str>,
) -> Result<Frame<'r>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut data: Frame = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row = make_row(record, col_names, col_types);
        data.push(row);
    }

    Ok(data)
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
    Field { name: name, value: value}
}

fn make_value(value: &str, given_type: &str) -> Value {
    match given_type {
        "str" => Value::Str(value.to_string()),
        "int" => {
            let value: i64 = value.parse().unwrap();
            Value::Int(value)
        },
        _ => Value::Str(value.to_string())
    }
}
