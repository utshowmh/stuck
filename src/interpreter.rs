use std::{collections::HashMap, io::stdin, process::exit};

use crate::{
    object::{Boolean, Function, Object},
    operation::{Operation, OperationType},
    tokenizer::Tokenizer,
};

pub struct Interpreter {
    program: Vec<Operation>,
    stack: Vec<Object>,
    variables: HashMap<String, Object>,
    registers_ia: Vec<Object>,
    register_f: Vec<usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            registers_ia: Vec::new(),
            register_f: Vec::new(),
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
                                if let Some(object) = self.registers_ia.pop() {
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

                                            Object::Boolean(boolean) => {
                                                self.stack.push(Object::Boolean(boolean.clone()))
                                            }

                                            Object::Function(function) => {
                                                self.stack.push(Object::Function(
                                                    Function::assign_called_from(
                                                        function.opening_block,
                                                        instruction_pointer + 1,
                                                    ),
                                                ));
                                                instruction_pointer = function.opening_block;
                                            }

                                            _ => {
                                                self.invalid_variable_type(&format!(
                                                    "in line {}. can use only number, string, boolean or function ('[]')",
                                                    operation.line
                                                ));
                                            }
                                        }
                                    } else {
                                        self.undefined_variable(&format!(
                                            "variable '{}' does not exist in line {}",
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
                                .invalid_type(&format!("expected string, found '{:?}'", operation)),
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
                                .invalid_type(&format!("expected number, found '{:?}'", operation)),
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::True => {
                    self.stack.push(Object::Boolean(Boolean::True));

                    instruction_pointer += 1;
                }

                OperationType::False => {
                    self.stack.push(Object::Boolean(Boolean::False));

                    instruction_pointer += 1;
                }

                OperationType::Function => {
                    if let Some(ending_block) = &operation.operand {
                        match ending_block {
                            Object::Reference(ending_block) => {
                                self.register_f.push(instruction_pointer);
                                instruction_pointer = ending_block.to_owned();
                            }

                            _ => {
                                self.invalid_reference(&format!(
                                    "invalid reference for 'function' (expected integer) in line {}",
                                    operation.line
                                ));
                            }
                        }
                    } else {
                        if let Some(opening_block) = self.register_f.pop() {
                            self.stack
                                .push(Object::Function(Function::new(opening_block)));
                            instruction_pointer += 1;
                        } else {
                            instruction_pointer = match self.stack.pop().unwrap() {
                                Object::Function(function) => function.called_from,
                                _ => {
                                    self.invalid_reference(&format!("could not figure out where the is being called from in line {}", operation.line));
                                    0
                                }
                            }
                        }
                    }
                }

                OperationType::Assignment => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "can not declare variable without a value in line {}",
                            operation.line
                        ));
                    }

                    let object = self.stack.pop().unwrap();
                    self.registers_ia.push(object);

                    instruction_pointer += 1;
                }

                OperationType::Plus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'+' operation requires two operand in line {}",
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
                                "'+' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Minus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'-' operation requires two operand in line {}",
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
                                "'-' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Multiplication => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'*' operation requires two operand in line {}",
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
                                "'*' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Division => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'/' operation requires two operand in line {}",
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
                                "'/' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Modulus => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'/' operation requires two operand in line {}",
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
                                "'/' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Equal => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'=' operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if a == b {
                        self.stack.push(Object::Boolean(Boolean::True));
                    } else {
                        self.stack.push(Object::Boolean(Boolean::False));
                    }

                    instruction_pointer += 1;
                }

                OperationType::Greater => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'>' operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            if y > x {
                                self.stack.push(Object::Boolean(Boolean::True));
                            } else {
                                self.stack.push(Object::Boolean(Boolean::False));
                            }
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "'>' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Less => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'>' operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Number(x), Object::Number(y)) => {
                            if y < x {
                                self.stack.push(Object::Boolean(Boolean::True));
                            } else {
                                self.stack.push(Object::Boolean(Boolean::False));
                            }
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "'<' is only usable with number in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Not => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "'!' operator requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::Boolean(x) => match x {
                            Boolean::True => self.stack.push(Object::Boolean(Boolean::False)),
                            Boolean::False => self.stack.push(Object::Boolean(Boolean::True)),
                        },
                        _ => {
                            self.invalid_type(&format!(
                                "'!' is only usable with boolean in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::And => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'&' operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Boolean(x), Object::Boolean(y)) => match (x, y) {
                            (Boolean::True, Boolean::True) => {
                                self.stack.push(Object::Boolean(Boolean::True))
                            }

                            _ => self.stack.push(Object::Boolean(Boolean::False)),
                        },

                        _ => {
                            self.invalid_type(&format!(
                                "'&' is only usable with boolean in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Or => {
                    if self.stack.len() < 2 {
                        self.stack_underflow(&format!(
                            "'|' operation requires two operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    match (a, b) {
                        (Object::Boolean(x), Object::Boolean(y)) => match (x, y) {
                            (Boolean::False, Boolean::False) => {
                                self.stack.push(Object::Boolean(Boolean::False))
                            }

                            _ => self.stack.push(Object::Boolean(Boolean::True)),
                        },

                        _ => {
                            self.invalid_type(&format!(
                                "'|' is only usable with boolean in line {}",
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
                            "'then' operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::Boolean(boolean) => {
                            if let Some(end_block) = &operation.operand {
                                match end_block {
                                    Object::Reference(number) => match boolean {
                                        Boolean::False => instruction_pointer = number.to_owned(),
                                        Boolean::True => instruction_pointer += 1,
                                    },
                                    _ => {
                                        self.invalid_reference(&format!(
                                            "invalid reference for 'then' (expected integer) in line {}",
                                            operation.line
                                        ));
                                    }
                                }
                            } else {
                                self.invalid_reference(&format!(
                                    "'then' does not have reference to it's 'end' or 'else' block in line {}",
                                    operation.line
                                ));
                            }
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "'then' doesn't have a reference associated with it in line {}",
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
                            _ => {
                                self.invalid_reference(&format!(
                                    "invalid reference for 'else' (expected integer) in line {}",
                                    operation.line
                                ));
                            }
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "'else' does not have reference to it's 'end' block in line {}",
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
                            "'do' operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::Boolean(boolean) => {
                            if let Some(end_block) = &operation.operand {
                                match end_block {
                                    Object::Reference(number) => match boolean {
                                        Boolean::False => instruction_pointer = number.to_owned(),
                                        Boolean::True => instruction_pointer += 1,
                                    },
                                    _ => {
                                        self.invalid_reference(&format!(
                                            "invalid reference for 'do' (expected integer) in line {}",
                                            operation.line
                                        ));
                                    }
                                }
                            } else {
                                self.invalid_reference(&format!(
                                    "'do' does not have reference to it's 'end' block in line {}",
                                    operation.line
                                ));
                            }
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "'then' doesn't have a reference associated with it in line {}",
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
                                    "can not use '{:?}' with 'end' (expected integer) in line {}",
                                    invalid_type, operation.line
                                ));
                            }
                        }
                    } else {
                        self.invalid_reference(&format!(
                            "'end' doesn't have a reference associated with it in line {}",
                            operation.line
                        ));
                    }
                }

                OperationType::Include => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "'include' operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::String(_) => {
                            todo!()
                        }
                        _ => {
                            self.invalid_type(&format!(
                                "'write' can use with only string in line {}",
                                operation.line
                            ));
                        }
                    }

                    instruction_pointer += 1;
                }

                OperationType::Read => {
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

                OperationType::Write => {
                    if self.stack.len() < 1 {
                        self.stack_underflow(&format!(
                            "'write' operation requires one operand in line {}",
                            operation.line
                        ));
                    }

                    let a = self.stack.pop().unwrap();
                    match a {
                        Object::String(string) => print!("{}", string),
                        Object::Number(number) => print!("{}", number),
                        Object::Boolean(boolean) => match boolean {
                            Boolean::True => print!("true"),
                            Boolean::False => print!("false"),
                        },
                        _ => {
                            self.invalid_type(&format!(
                                "'write' can use with only number, string and boolean in line {}",
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
        self.error("StackUnderflow", message);
    }

    fn undefined_variable(&self, message: &str) {
        self.error("UndefinedVariable", message);
    }

    fn invalid_reference(&self, message: &str) {
        self.error("InvalidReference", message);
    }

    fn invalid_type(&self, message: &str) {
        self.error("InvalidType", message);
    }

    fn invalid_variable_type(&self, message: &str) {
        self.error("InvalidVariableType", message);
    }

    fn error(&self, e_type: &str, message: &str) {
        eprintln!("{}: {}.", e_type, message);
        exit(1);
    }
}
