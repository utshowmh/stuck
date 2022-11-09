#[derive(Debug, PartialEq, Clone)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub opening_block: usize,
    pub called_from: usize,
}

impl Function {
    pub fn new(opening_block: usize) -> Self {
        Self {
            opening_block,
            called_from: 0,
        }
    }

    pub fn assign_called_from(opening_block: usize, called_from: usize) -> Self {
        Self {
            opening_block,
            called_from,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(Boolean),
    Function(Function),
    Reference(usize),
}
