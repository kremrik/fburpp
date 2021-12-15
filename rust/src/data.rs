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
