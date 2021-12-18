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

pub fn select<'s>(row: Row<'s>, fields: &Vec<String>) -> Row<'s> {
    let mut output: Row = Vec::new();

    for field in row {
        if fields.contains(&field.name.to_string()) {
            output.push(field);
        }
    }

    output
}
