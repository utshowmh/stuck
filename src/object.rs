#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Object {
    Identifier(String),
    Number(i64),
}

pub type Number = i64;
