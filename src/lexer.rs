use std::process::exit;

use crate::{
    global::Integer,
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

                    "end" => {
                        program.push(Operation::new(OperationType::End, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::End,
                            None,
                            self.line_number,
                        ));
                    }

                    "." => {
                        program.push(Operation::new(OperationType::Dump, None, self.line_number));
                        crossrefernced_program.push(Operation::new(
                            OperationType::Dump,
                            None,
                            self.line_number,
                        ));
                    }

                    token => {
                        if let Ok(number) = token.parse() {
                            program.push(Operation::new(
                                OperationType::Push,
                                Some(number),
                                self.line_number,
                            ));
                            crossrefernced_program.push(Operation::new(
                                OperationType::Push,
                                Some(number),
                                self.line_number,
                            ));
                        } else {
                            self.error(&format!("Unknown Token `{:#?}`", token))
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

                OperationType::End => {
                    if let Some(opening_block) = block_references.pop() {
                        let opening_block = &mut crossreferenced_program[opening_block];

                        match &opening_block.op_type {
                            OperationType::Then => {
                                opening_block.operand = Some(operation_index as Integer);
                            }

                            opening_block => {
                                self.error(&format!("Can't close `end` with {:#?}", opening_block));
                            }
                        }
                    } else {
                        self.error("Unexted `end`");
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
