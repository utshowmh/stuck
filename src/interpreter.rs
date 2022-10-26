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
                    if let Some(operand) = operation.operand {
                        self.stack.push(operand);
                    }

                    instruction_pointer += 1;
                }

                OperationType::Dup => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`dup` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    self.stack.push(a);
                    self.stack.push(a);

                    instruction_pointer += 1;
                }

                OperationType::Plus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`+` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);

                    instruction_pointer += 1;
                }

                OperationType::Minus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`-` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);

                    instruction_pointer += 1;
                }

                OperationType::Equal => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`=` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((a == b) as Integer);

                    instruction_pointer += 1;
                }

                OperationType::Greater => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`>` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b > a) as Integer);

                    instruction_pointer += 1;
                }

                OperationType::If => {
                    instruction_pointer += 1;
                }

                OperationType::Then => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`then` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    if let Some(end_block) = operation.operand {
                        if a == 0 {
                            instruction_pointer = end_block as usize;
                        } else {
                            instruction_pointer += 1;
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "`then` does not have reference to it's `end` or `else` block in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::Else => {
                    if let Some(end_block) = operation.operand {
                        instruction_pointer = end_block as usize;
                    } else {
                        self.invalid_reference(&format!(
                            "`else` does not have reference to it's `end` block in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::While => {
                    instruction_pointer += 1;
                }

                OperationType::Do => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`do` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    if let Some(end_block) = operation.operand {
                        if a == 0 {
                            instruction_pointer = (end_block) as usize;
                        } else {
                            instruction_pointer += 1;
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "`do` does not have reference to it's `end` block in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::End => {
                    if let Some(starting_block) = operation.operand {
                        instruction_pointer = starting_block as usize;
                    } else {
                        instruction_pointer += 1;
                    }
                }

                OperationType::Dump => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`.` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    println!("{}", a);

                    instruction_pointer += 1;
                }
            }
        }
    }

    fn stack_underflow(&self, message: &str) {
        self.error("StackUnderflow", message);
    }

    fn invalid_reference(&self, message: &str) {
        self.error("Invalid Reference", message);
    }

    fn error(&self, e_type: &str, message: &str) {
        eprintln!("{}: {}", e_type, message);
        exit(1);
    }
}
