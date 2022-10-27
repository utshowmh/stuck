use std::process::exit;

use crate::{
    object::Object,
    operation::{Operation, OperationType},
};

pub struct Lexer {
    source: String,
    line_number: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line_number: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Operation> {
        let mut program = Vec::new();
        let mut crossrefernced_program = Vec::new();
        let source = self.source.trim().split("\n");

        'line: for line in source {
            for token in line.trim().split(" ") {
                match token {
                    " " | "\t" | "\r" | "\n" | "" => continue,

                    "#" => {
                        self.line_number += 1;
                        continue 'line;
                    }

                    "+" => {
                        program.push(Operation::new(OperationType::Plus, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Plus,
                            None,
                            self.line_number,
                        ));
                    }

                    "-" => {
                        program.push(Operation::new(OperationType::Minus, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Minus,
                            None,
                            self.line_number,
                        ));
                    }

                    "*" => {
                        program.push(Operation::new(
                            OperationType::Multiplication,
                            None,
                            self.line_number,
                        ));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Multiplication,
                            None,
                            self.line_number,
                        ));
                    }

                    "/" => {
                        program.push(Operation::new(
                            OperationType::Division,
                            None,
                            self.line_number,
                        ));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Division,
                            None,
                            self.line_number,
                        ));
                    }

                    "=" => {
                        program.push(Operation::new(OperationType::Equal, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Equal,
                            None,
                            self.line_number,
                        ));
                    }

                    ">" => {
                        program.push(Operation::new(
                            OperationType::Greater,
                            None,
                            self.line_number,
                        ));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Greater,
                            None,
                            self.line_number,
                        ));
                    }

                    "<" => {
                        program.push(Operation::new(OperationType::Less, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Less,
                            None,
                            self.line_number,
                        ));
                    }

                    "dump" => {
                        program.push(Operation::new(OperationType::Dump, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Dump,
                            None,
                            self.line_number,
                        ));
                    }

                    "var" => {
                        program.push(Operation::new(
                            OperationType::Variable,
                            None,
                            self.line_number,
                        ));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Variable,
                            None,
                            self.line_number,
                        ));
                    }

                    "if" => {
                        program.push(Operation::new(OperationType::If, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::If,
                            None,
                            self.line_number,
                        ));
                    }

                    "then" => {
                        program.push(Operation::new(OperationType::Then, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Then,
                            None,
                            self.line_number,
                        ));
                    }

                    "else" => {
                        program.push(Operation::new(OperationType::Else, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Else,
                            None,
                            self.line_number,
                        ));
                    }

                    "while" => {
                        program.push(Operation::new(OperationType::While, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::While,
                            None,
                            self.line_number,
                        ));
                    }

                    "do" => {
                        program.push(Operation::new(OperationType::Do, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Do,
                            None,
                            self.line_number,
                        ));
                    }

                    "end" => {
                        program.push(Operation::new(OperationType::End, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::End,
                            None,
                            self.line_number,
                        ));
                    }

                    token => {
                        if let Ok(number) = token.parse::<i64>() {
                            program.push(Operation::new(
                                OperationType::Number,
                                Some(Object::Number(number)),
                                self.line_number,
                            ));
                            crossrefernced_program.push(Operation::new(
                                OperationType::Number,
                                Some(Object::Number(number)),
                                self.line_number,
                            ));
                        } else {
                            program.push(Operation::new(
                                OperationType::Identifier,
                                Some(Object::Identifier(token.to_string())),
                                self.line_number,
                            ));
                            crossrefernced_program.push(Operation::new(
                                OperationType::Identifier,
                                Some(Object::Identifier(token.to_string())),
                                self.line_number,
                            ));
                        }
                    }
                }
            }
            self.line_number += 1
        }

        self.crossrefrence_blocks(&mut crossrefernced_program, &program);
        crossrefernced_program
    }
}

impl Lexer {
    fn crossrefrence_blocks(
        &self,
        crossreferenced_program: &mut Vec<Operation>,
        program: &Vec<Operation>,
    ) {
        let mut block_references = Vec::new();

        for (operation_index, operation) in program.iter().enumerate() {
            match operation.op_type {
                OperationType::Then => block_references.push(operation_index),

                OperationType::While => block_references.push(operation_index),

                OperationType::Do => block_references.push(operation_index),

                OperationType::Else => {
                    if let Some(if_block) = block_references.pop() {
                        block_references.push(operation_index);
                        let if_block = &mut crossreferenced_program[if_block];

                        match &if_block.op_type {
                            OperationType::Then => {
                                if_block.operand =
                                    Some(Object::Number((operation_index + 1) as i64));
                            }

                            opening_block => {
                                self.error(&format!(
                                    "Can't use `else` with `{:#?}`",
                                    opening_block
                                ));
                            }
                        }
                    } else {
                        self.error("Unexpected `else`");
                    }
                }

                OperationType::End => {
                    if let Some(opening_block) = block_references.pop() {
                        let opening_block = &mut crossreferenced_program[opening_block];

                        match &opening_block.op_type {
                            OperationType::Then => {
                                opening_block.operand =
                                    Some(Object::Number((operation_index + 1) as i64));
                            }

                            OperationType::Do => {
                                opening_block.operand =
                                    Some(Object::Number((operation_index + 1) as i64));
                                let end = &mut crossreferenced_program[operation_index];
                                let while_block = block_references.pop().unwrap();
                                end.operand = Some(Object::Number((while_block) as i64));
                            }

                            OperationType::Else => {
                                opening_block.operand =
                                    Some(Object::Number((operation_index + 1) as i64));
                            }

                            opening_block => {
                                self.error(&format!(
                                    "Can't close `end` with `{:#?}`",
                                    opening_block
                                ));
                            }
                        }
                    } else {
                        self.error("Unexpected `end`");
                    }
                }

                _ => continue,
            }
        }
    }

    fn error(&self, message: &str) {
        eprintln!("LexicalError: {} in line {}", message, self.line_number);
        exit(1);
    }
}
