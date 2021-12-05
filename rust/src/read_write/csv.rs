use csv::StringRecord;

use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i64),
}

fn map_type(value: &str, given_type: &str) -> Value {
    match given_type {
        "str" => Value::Str(value.to_string()),
        "int" => {
            let value: i64 = value.parse().unwrap();
            Value::Int(value)
        },
        _ => Value::Str(value.to_string())
    }
}

fn map_schema(
    record: StringRecord,
    schema: &Vec<&str>,
) -> Vec<Value> {
    let mut row: Vec<Value> = Vec::new();
    
    let val_typ = record.iter().zip(schema.iter());
    for (val, typ) in val_typ {
        let r = map_type(val, typ);
        row.push(r);
    }

    row
}

pub fn reader(
    schema: &Vec<&str>
) -> Result<Vec<Vec<Value>>, Box<dyn Error>> {
    let mut data: Vec<Vec<Value>> = Vec::new();

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        let values = map_schema(record, &schema);
        data.push(values);
    }
    Ok(data)
}
