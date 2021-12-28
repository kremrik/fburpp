use crate::job::{Filter, Select};

#[derive(Debug)]
pub struct Field<'f> {
    pub name: &'f str,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i64),
}

pub type Row<'r> = Vec<Field<'r>>;

pub fn select<'s>(row: Row<'s>, s: &Option<Select>) -> Row<'s> {
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

pub fn filter<'f>(
    row: Row<'f>, f: &Option<Vec<Filter>>
) -> Option<Row<'f>> {
    match f {
        None => Some(row),
        Some(_filts) => Some(row)
    }
}
