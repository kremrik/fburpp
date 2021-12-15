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
