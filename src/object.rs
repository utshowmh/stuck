#[derive(Debug, PartialEq, Clone)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(Boolean),
    Reference(usize),
}
