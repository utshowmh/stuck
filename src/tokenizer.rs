use std::{collections::HashMap, process::exit};

use crate::{
    object::Object,
    operation::{Operation, OperationType},
};

pub struct Tokenizer {
    source: Vec<u8>,
    source_len: usize,
    operations: Vec<Operation>,
    keywords: HashMap<String, OperationType>,
    index: usize,
    current_charecter: Option<char>,
    line_number: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.as_bytes().to_vec(),
            source_len: source.len(),
            operations: Vec::new(),
            keywords: HashMap::new(),
            index: 0,
            current_charecter: None,
            line_number: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Operation> {
        self.init_keywords();
        self.scan();
        return self.crossreference_operations();
    }
}

impl Tokenizer {
    fn scan(&mut self) {
        self.advance();

        while let Some(current_charecter) = self.current_charecter {
            match current_charecter {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }

                '\n' => {
                    self.advance();
                    self.line_number += 1;
                }

                '#' => self.make_comment(),

                '+' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Plus,
                        None,
                        self.line_number,
                    ));
                }

                '-' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Minus,
                        None,
                        self.line_number,
                    ));
                }

                '*' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Multiplication,
                        None,
                        self.line_number,
                    ));
                }

                '/' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Division,
                        None,
                        self.line_number,
                    ));
                }

                '%' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Modulus,
                        None,
                        self.line_number,
                    ));
                }

                '=' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Equal,
                        None,
                        self.line_number,
                    ));
                }

                '>' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Greater,
                        None,
                        self.line_number,
                    ));
                }

                '<' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Less,
                        None,
                        self.line_number,
                    ));
                }

                '!' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Not,
                        None,
                        self.line_number,
                    ));
                }

                '&' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::And,
                        None,
                        self.line_number,
                    ));
                }

                '|' => {
                    self.advance();
                    self.operations
                        .push(Operation::new(OperationType::Or, None, self.line_number));
                }

                '@' => {
                    self.advance();
                    self.operations.push(Operation::new(
                        OperationType::Assignment,
                        None,
                        self.line_number,
                    ));
                }

                '"' => {
                    self.advance();
                    self.make_string();
                }

                token => {
                    if token.is_digit(10) {
                        self.make_number();
                    } else if token.is_alphabetic() {
                        self.make_identifier();
                    } else {
                        self.error(&format!("invalid token `{}`", token));
                    }
                }
            }
        }
    }

    fn crossreference_operations(&mut self) -> Vec<Operation> {
        let mut block_references = Vec::new();
        let mut crossreferened_operations = self.operations.clone();

        for (operation_index, operation) in self.operations.iter().enumerate() {
            match operation.op_type {
                OperationType::If => block_references.push(operation_index),

                OperationType::Then => block_references.push(operation_index),

                OperationType::While => block_references.push(operation_index),

                OperationType::Do => block_references.push(operation_index),

                OperationType::Else => {
                    if let Some(then_block) = block_references.pop() {
                        let operation_then = &self.operations[then_block];
                        match &operation_then.op_type {
                            OperationType::Then => {
                                if let Some(if_else_block) = block_references.pop() {
                                    let operation_if = &self.operations[if_else_block];
                                    match &operation_if.op_type {
                                        OperationType::If => {
                                            crossreferened_operations[then_block].operand =
                                                Some(Object::Reference(operation_index + 1));
                                            block_references.push(operation_index);
                                        }

                                        OperationType::Else => {
                                            crossreferened_operations[then_block].operand =
                                                Some(Object::Reference(operation_index + 1));
                                            crossreferened_operations[if_else_block].operand =
                                                Some(Object::Reference(operation_index + 1));
                                            block_references.push(operation_index);
                                        }

                                        invalid_block => self.error(&format!(
                                            "can't end `else` with `{:?}`",
                                            invalid_block
                                        )),
                                    }
                                }
                            }

                            invalid_block => {
                                self.error(&format!("can't end `else` with `{:?}`", invalid_block))
                            }
                        }
                    } else {
                        self.error(&format!("invalid `else`"));
                    }
                }

                OperationType::End => {
                    if let Some(opening_block) = block_references.pop() {
                        let operation = &self.operations[opening_block];
                        match &operation.op_type {
                            OperationType::Then => {
                                if let Some(if_block) = block_references.pop() {
                                    let operation = &self.operations[if_block];
                                    match &operation.op_type {
                                        OperationType::If => {
                                            crossreferened_operations[opening_block].operand =
                                                Some(Object::Reference(operation_index + 1));
                                            crossreferened_operations[operation_index].operand =
                                                Some(Object::Reference(operation_index + 1));
                                        }

                                        invalid_block => self.error(&format!(
                                            "can't end `end` with `{:?}`",
                                            invalid_block
                                        )),
                                    }
                                } else {
                                    self.error(&format!("unexpected `then`"));
                                }
                            }

                            OperationType::Else => {
                                crossreferened_operations[opening_block].operand =
                                    Some(Object::Reference(operation_index + 1));
                                crossreferened_operations[operation_index].operand =
                                    Some(Object::Reference(operation_index + 1));
                            }

                            OperationType::Do => {
                                if let Some(while_block) = block_references.pop() {
                                    let operation = &self.operations[while_block];
                                    match &operation.op_type {
                                        OperationType::While => {
                                            crossreferened_operations[opening_block].operand =
                                                Some(Object::Reference(operation_index + 1));
                                            crossreferened_operations[operation_index].operand =
                                                Some(Object::Reference(while_block));
                                        }

                                        invalid_block => self.error(&format!(
                                            "can't end `end` with `{:?}`",
                                            invalid_block
                                        )),
                                    }
                                } else {
                                    self.error(&format!("unexpected `do`"));
                                }
                            }

                            invalid_block => {
                                self.error(&format!("can't end `end` with `{:?}`", invalid_block))
                            }
                        }
                    } else {
                        self.error(&format!("unexpexted `end`"));
                    }
                }

                _ => {}
            }
        }

        crossreferened_operations
    }

    fn advance(&mut self) {
        if self.index < self.source_len {
            self.current_charecter = Some(self.source[self.index] as char);
            self.index += 1;
        } else {
            self.current_charecter = None;
        }
    }

    fn make_comment(&mut self) {
        while let Some(current_charecter) = self.current_charecter {
            self.advance();
            if current_charecter == '\n' {
                self.line_number += 1;
                return;
            }
        }
    }

    fn make_number(&mut self) {
        let mut number = String::new();
        while let Some(current_charecter) = self.current_charecter {
            if current_charecter.is_digit(10) || current_charecter == '.' {
                number.push(current_charecter);
                self.advance();
            } else {
                let number = number.parse::<f64>().unwrap_or_else(|_| {
                    self.error(&format!("Can't convert `{}` to a number", number));
                    exit(1);
                });
                self.operations.push(Operation::new(
                    OperationType::Number,
                    Some(Object::Number(number)),
                    self.line_number,
                ));
                return;
            }
        }
    }

    fn make_identifier(&mut self) {
        let mut identifier = String::new();
        while let Some(current_charecter) = self.current_charecter {
            if current_charecter.is_alphanumeric() || current_charecter == '_' {
                identifier.push(current_charecter);
                self.advance();
            } else {
                if let Some(keyword) = self.keywords.get(&identifier) {
                    let keyword = keyword;
                    self.operations
                        .push(Operation::new(keyword.clone(), None, self.line_number));
                    return;
                } else {
                    self.operations.push(Operation::new(
                        OperationType::Identifier,
                        Some(Object::Identifier(identifier)),
                        self.line_number,
                    ));
                    return;
                }
            }
        }
    }

    fn make_string(&mut self) {
        let mut string = String::new();
        while let Some(current_charecter) = self.current_charecter {
            string.push(current_charecter);
            self.advance();
            if current_charecter == '"' {
                let string = string
                    .strip_suffix("\"")
                    .unwrap()
                    .replace("\\n", "\n")
                    .replace("\\t", "\t");
                self.operations.push(Operation::new(
                    OperationType::String,
                    Some(Object::String(string.to_string())),
                    self.line_number,
                ));
                return;
            } else if current_charecter == '\n' {
                break;
            }
        }
        self.error("untermated string");
    }

    fn init_keywords(&mut self) {
        self.keywords
            .insert("true".to_string(), OperationType::True);
        self.keywords
            .insert("false".to_string(), OperationType::False);

        self.keywords
            .insert("read".to_string(), OperationType::Read);
        self.keywords
            .insert("write".to_string(), OperationType::Write);
        self.keywords
            .insert("writeln".to_string(), OperationType::Writeln);

        self.keywords.insert("if".to_string(), OperationType::If);
        self.keywords
            .insert("then".to_string(), OperationType::Then);
        self.keywords
            .insert("else".to_string(), OperationType::Else);

        self.keywords
            .insert("while".to_string(), OperationType::While);
        self.keywords.insert("do".to_string(), OperationType::Do);

        self.keywords.insert("end".to_string(), OperationType::End);
    }
}

impl Tokenizer {
    fn error(&self, message: &str) {
        eprintln!("Lexical Error: {} in line {}", message, self.line_number);
        exit(1);
    }
}
