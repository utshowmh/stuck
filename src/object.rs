#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Object {
    Identifier(String),
    Number(i64),
}

pub type Number = i64;
