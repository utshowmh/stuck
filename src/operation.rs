use crate::object::Object;

#[derive(Debug)]
pub enum OperationType {
    Identifier,
    Number,

    Variable,

    Plus,
    Minus,
    Multiplication,
    Division,

    Equal,
    Greater,
    Less,

    If,
    Then,
    Else,
    While,
    Do,
    End,

    Dump,
}

#[derive(Debug)]
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
