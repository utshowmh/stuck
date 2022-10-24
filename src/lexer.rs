use std::process::exit;

use crate::operation::{Operation, OperationType};

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
                    }

                    "-" => {
                        program.push(Operation::new(OperationType::Minus, None, self.line_number));
                    }

                    "." => {
                        program.push(Operation::new(OperationType::Dump, None, self.line_number));
                    }

                    token => {
                        if let Ok(number) = token.parse() {
                            program.push(Operation::new(
                                OperationType::Push,
                                Some(number),
                                self.line_number,
                            ));
                        } else {
                            self.error(&format!(
                                "Unknown Token `{:#?}` in line {}",
                                token, self.line_number
                            ))
                        }
                    }
                }
            }
            self.line_number += 1
        }

        program
    }
}

impl Lexer {
    fn error(&self, message: &str) {
        eprintln!("LexicalError: {}", message);
        exit(1);
    }
}
