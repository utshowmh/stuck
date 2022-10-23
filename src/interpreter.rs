use std::{fs::read_to_string, process::exit};

use crate::{
    global::Integer,
    lexer::Lexer,
    operation::{Operation, OperationType},
};

pub struct Interpreter {
    program: Vec<Operation>,
    stack: Vec<Integer>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self, source_path: &str) {
        let source = read_to_string(source_path).unwrap_or_else(|err| {
            eprintln!("ERROR: {:#?}", err);
            exit(1);
        });
        let mut lexer = Lexer::new(source);
        self.program.append(&mut lexer.lex());

        self.interpret();
    }
}

impl Interpreter {
    fn interpret(&mut self) {
        let mut instruction_pointer = 0;

        while instruction_pointer < self.program.len() {
            let operation = &self.program[instruction_pointer];

            match operation.op_type {
                OperationType::Push => {
                    instruction_pointer += 1;

                    if let Some(operand) = operation.operand {
                        self.stack.push(operand);
                    }
                }

                OperationType::Plus => {
                    instruction_pointer += 1;

                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`+` operation requires two operand; in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }

                OperationType::Minus => {
                    instruction_pointer += 1;

                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`-` operation requires two operand; in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);
                }

                OperationType::Multiplication => {
                    instruction_pointer += 1;

                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`*` operation requires two operand; in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }

                OperationType::Division => {
                    instruction_pointer += 1;

                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`/` operation requires two operand; in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b / a);
                }

                OperationType::Dump => {
                    instruction_pointer += 1;

                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`.` operation requires one operand; in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    println!("{}", a);
                }
            }
        }
    }

    fn stack_underflow(&self, message: &str) {
        self.error("StackUnderflow", message);
    }

    fn error(&self, e_type: &str, message: &str) {
        eprintln!("{}: {}", e_type, message);
        exit(1);
    }
}
