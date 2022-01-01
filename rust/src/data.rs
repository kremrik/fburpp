use crate::job::{Filter, Select};

pub type Row = Vec<Field>;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Null,
}

pub fn select(row: Row, s: &Option<Select>) -> Row {
    match s {
        None => row,
        Some(select) => {
            let mut output: Row = Vec::new();
            let fields = &select.fields;

            for field in row {
                if fields.contains(&field.name.to_string()) {
                    output.push(field);
                }
            }

            output
        },
    }
}

pub fn filter(
    row: Row, f: &Option<Vec<Filter>>
) -> Option<Row> {
    match f {
        None => Some(row),
        Some(_filts) => Some(row)
    }
}
