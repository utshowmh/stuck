use crate::object::Object;

#[derive(Debug, Clone)]
pub enum OperationType {
    Identifier,
    Number,
    String,

    Assignment,

    Plus,
    Minus,
    Multiplication,
    Division,
    Modulus,

    Equal,
    Greater,
    Less,

    And,

    If,
    Then,
    Else,
    While,
    Do,
    End,

    Input,
    Print,
    Println,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub op_type: OperationType,
    pub operand: Option<Object>,
    pub line: usize,
}

impl Operation {
    pub fn new(op_type: OperationType, operand: Option<Object>, line: usize) -> Self {
        Self {
            op_type,
            operand,
            line,
        }
    }
}
