pub type Number = f64;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Identifier(String),
    Number(Number),
    String(String),
    Reference(usize),
}
