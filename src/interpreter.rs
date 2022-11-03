use std::{collections::HashMap, process::exit, io::stdin};

use crate::{
    object::{Number, Object},
    operation::{Operation, OperationType},
    tokenizer::Tokenizer,
};

pub struct Interpreter {
    program: Vec<Operation>,
    stack: Vec<Object>,
    variables: HashMap<String, Object>,
    registers: Vec<Object>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            registers: Vec::new(),
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut tokenizer = Tokenizer::new(source);
        self.program = tokenizer.scan_tokens().to_vec();

        self.interpret();
    }
}

impl Interpreter {
    fn interpret(&mut self) {
        let mut instruction_pointer = 0;

        while instruction_pointer < self.program.len() {
            let operation = &self.program[instruction_pointer];

            match operation.op_type {
                OperationType::Identifier => {
                    if let Some(variable) = &operation.operand {
                        match variable {
                            Object::Identifier(identifier) => {
                                if let Some(object) = self.registers.pop() {
                                    self.variables.insert(identifier.to_string(), object);
                                } else {
                                    if let Some(object) = self.variables.get(identifier) {
                                        match object {
                                            Object::Number(number) => {
                                                self.stack.push(Object::Number(number.to_owned()))
                                            }

                                            Object::String(string) => {
                                                self.stack.push(Object::String(string.clone()))
                                            }

                                            _ => {
                                                self.unknown_error(&format!(
                                                    "in line {}",
                                                    operation.line
                                                ));
                                            }
                                        }
                                    } else {
                                        self.undefined_variable(&format!(
                                            "variable `{}` does not exist in line {}",
                                            identifier, operation.line
                                        ));
                                    }
                                }
                            }

                            _ => {}
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::String => {
                    if let Some(operand) = &operation.operand {
                        match operand {
                            Object::String(string) => {
                                self.stack.push(Object::String(string.clone()))
                            }

                            _ => self
                                .invalid_type(&format!("expected string, found `{:?}`", operation)),
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Number => {
                    if let Some(operand) = &operation.operand {
                        match operand {
                            Object::Number(number) => {
                                self.stack.push(Object::Number(number.to_owned()))
                            }

                            _ => self
                                .invalid_type(&format!("expected number, found `{:?}`", operation)),
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Variable => {
                    instruction_pointer += 1;
                }

                OperationType::Assignment => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "can't declare variable without a value in line {}",
                            operation.line
                        ));
                    }

                    let object = self.stack.pop().unwrap();
                    self.registers.push(object);

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
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack.push(Object::Number(x + y));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`+` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

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
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack.push(Object::Number(y - x));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`-` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

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
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack.push(Object::Number(x * y));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`*` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

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
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack.push(Object::Number(y / x));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`/` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Modulus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`/` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack.push(Object::Number(y % x));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`/` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

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
                    let a = ((a == b) as usize) as Number;
                    self.stack.push(Object::Number(a));

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
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack
                                .push(Object::Number(((y > x) as usize) as Number));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`>` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Less => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "`>` operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            self.stack
                                .push(Object::Number(((y < x) as usize) as Number));
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "`<` is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

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
                    match a {
                        Object::Number(boolean) => {
                            if let Some(end_block) = &operation.operand {
                                match end_block {
                                    Object::Reference(number) => {
                                        if boolean == 0. {
                                            instruction_pointer = number.to_owned();
                                        } else {
                                            instruction_pointer += 1;
                                        }
                                    }
                                    invalid_type => {
                                        self.invalid_type(&format!(
                                            "can't use `{:?}` with `then` (expected integer) in line {}",
                                            invalid_type, operation.line
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
                        _ => {
                            self.invalid_type(&format!(
                                "`then` doesn't have a reference associated with it in line {}",
                                operation.line
                            ));
                        }
                    }
                }

                OperationType::Else => {
                    if let Some(end_block) = &operation.operand {
                        match end_block {
                            Object::Reference(number) => {
                                instruction_pointer = number.to_owned();
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "can't use `{:?}` with `else` (expected integer) in line {}",
                                    invalid_type, operation.line
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
                    match a {
                        Object::Number(boolean) => {
                            if let Some(end_block) = &operation.operand {
                                match end_block {
                                    Object::Reference(number) => {
                                        if boolean == 0. {
                                            instruction_pointer = number.to_owned();
                                        } else {
                                            instruction_pointer += 1;
                                        }
                                    }
                                    invalid_type => {
                                        self.invalid_type(&format!(
                                            "can't use `{:?}` with `then` (expected integer) in line {}",
                                            invalid_type, operation.line
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
                        _ => {
                            self.invalid_type(&format!(
                                "`then` doesn't have a reference associated with it in line {}",
                                operation.line
                            ));
                        }
                    }
                }

                OperationType::End => {
                    if let Some(starting_block) = &operation.operand {
                        match starting_block {
                            Object::Reference(number) => {
                                instruction_pointer = number.to_owned();
                            }
                            invalid_type => {
                                self.invalid_type(&format!(
                                    "can't use `{:?}` with `end` (expected integer) in line {}",
                                    invalid_type, operation.line
                                ));
                            }
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "`end` doesn't have a reference associated with it in line {}", 
                            operation.line
                        ));
                    }
                }

                OperationType::Input => {
                    let mut object = String::new();
                    stdin().read_line(&mut object).unwrap_or_else(|err| {
                        eprintln!("Error: {:#?}", err);
                        exit(1);
                    });
                    let object = object.trim();
                    if let Ok(number) = object.parse() {
                        self.stack.push(Object::Number(number));
                    } else {
                        self.stack.push(Object::String(object.to_string()));
                    }

                    instruction_pointer += 1;
                }

                OperationType::Print => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`print` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::String(string) => print!("{}", string),
                        Object::Number(number) => print!("{}", number),
                        _ => {
                            self.invalid_type(&format!(
                                "`print` can use with only number or string in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Println => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "`print` operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::String(string) => println!("{}", string),
                        Object::Number(number) => println!("{}", number),
                        _ => {
                            self.invalid_type(&format!(
                                "`print` can use with only number or string in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }
            }
        }
    }
}

impl Interpreter {
    fn stack_underflow(&self, message: &str) {
        self.error("Stack Underflow", message);
    }

    fn undefined_variable(&self, message: &str) {
        self.error("Undefined Variable", message);
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
