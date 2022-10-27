use std::{collections::HashMap, fs::read_to_string, process::exit};

use crate::{
    lexer::Lexer,
    object::{Number, Object},
    operation::{Operation, OperationType},
};

pub struct Interpreter {
    program: Vec<Operation>,
    stack: Vec<Number>,
    variables: HashMap<String, Object>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
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
                OperationType::Number => {
                    if let Some(operand) = &operation.operand {
                        match operand {
                            Object::Number(number) => self.stack.push(number.to_owned()),

                            _ => self
                                .invalid_type(&format!("expected number, found `{:?}`", operation)),
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Dump => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`dump` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    println!("{:?}", a);

                    instruction_pointer += 1;
                }

                OperationType::Identifier => {
                    match &operation.operand.as_ref().unwrap() {
                        Object::Identifier(identifier) => {
                            if let Some(variable) = self.variables.get(&identifier.to_string()) {
                                match variable {
                                    Object::Number(number) => self.stack.push(number.to_owned()),
                                    _ => {}
                                }
                            } else {
                                let a = self.stack.pop().unwrap_or_else(|| {
                                    self.stack_underflow("`identifier` requires an `object`.");
                                    exit(1);
                                });
                                self.variables
                                    .insert(identifier.to_string(), Object::Number(a));
                            }
                        }
                        _ => {
                            self.unknown_error(&format!(
                                "could not recognige the identifier in line {}",
                                operation.line
                            ));
                            exit(1);
                        }
                    };

                    instruction_pointer += 1;
                }

                OperationType::Variable => {
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

                OperationType::Multiplication => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`*` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a * b);

                    instruction_pointer += 1;
                }

                OperationType::Division => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`/` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b / a);

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
                    self.stack.push((a == b) as Number);

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
                    self.stack.push((b > a) as Number);

                    instruction_pointer += 1;
                }

                OperationType::Less => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`<` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b < a) as Number);

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
                    if let Some(end_block) = &operation.operand {
                        match end_block {
                            Object::Number(number) => {
                                if a == 0 {
                                    instruction_pointer = number.to_owned() as usize;
                                } else {
                                    instruction_pointer += 1;
                                }
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "expected integer, found `{:?}`",
                                    invalid_type
                                ));
                            }
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "`then` does not have reference to it's `end` or `else` block in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::Else => {
                    if let Some(end_block) = &operation.operand {
                        match end_block {
                            Object::Number(number) => {
                                instruction_pointer = number.to_owned() as usize;
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "expected integer, found `{:?}`",
                                    invalid_type
                                ));
                            }
                        }
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
                    if let Some(end_block) = &operation.operand {
                        match end_block {
                            Object::Number(number) => {
                                if a == 0 {
                                    instruction_pointer = number.to_owned() as usize;
                                } else {
                                    instruction_pointer += 1;
                                }
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "expected integer, found `{:?}`",
                                    invalid_type
                                ));
                            }
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "`do` does not have reference to it's `end` block in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::End => {
                    if let Some(starting_block) = &operation.operand {
                        match starting_block {
                            Object::Number(number) => {
                                instruction_pointer = number.to_owned() as usize;
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "expected integer, found `{:?}`",
                                    invalid_type
                                ));
                            }
                        }
                    } else {
                        instruction_pointer += 1;
                    }
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

    fn invalid_type(&self, message: &str) {
        self.error("Invalid Type", message);
    }

    fn unknown_error(&self, message: &str) {
        self.error("Unknown Error", message);
    }

    fn error(&self, e_type: &str, message: &str) {
        eprintln!("{}: {}", e_type, message);
        exit(1);
    }
}
