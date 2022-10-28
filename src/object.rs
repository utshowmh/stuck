pub type Number = f64;

#[derive(Debug, PartialEq)]
pub enum Object {
    Identifier(String),
    Number(Number),
}
