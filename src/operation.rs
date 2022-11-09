use crate::object::Object;

#[derive(Debug, Clone)]
pub enum OperationType {
    Identifier,

    Number,
    String,
    True,
    False,

    Function,

    Assignment,

    Plus,
    Minus,
    Multiplication,
    Division,
    Modulus,

    Equal,
    Greater,
    Less,

    Not,
    And,
    Or,

    If,
    Then,
    Else,
    While,
    Do,
    End,

    Read,
    Write,
    Writeln,
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
