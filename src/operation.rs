use crate::global::Integer;

#[derive(Debug)]
pub enum OperationType {
    Push,
    Plus,
    Minus,
    If,
    Then,
    Else,
    End,
    Dump,
}

#[derive(Debug)]
pub struct Operation {
    pub op_type: OperationType,
    pub operand: Option<Integer>,
    pub line: usize,
}

impl Operation {
    pub fn new(op_type: OperationType, operand: Option<Integer>, line: usize) -> Self {
        Self {
            op_type,
            operand,
            line,
        }
    }
}
