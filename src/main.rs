mod interpreter;
mod object;
mod operation;
mod tokenizer;

use std::{env::args, fs::read_to_string, process::exit};

use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => {
            match args[1].as_str() {
                "help" => {
                    help(None);
                }
                source_path => {
                    let mut interpreter = Interpreter::new();
                    let source = read_to_string(source_path).unwrap_or_else(|err| {
                        eprintln!("Error: {:#?}", err);
                        exit(2);
                    });
                    interpreter.run(&source);
                }
            };
        }
        _ => {
            help(Some("invalid subcommands"));
        }
    }
}

fn help(message: Option<&str>) {
    println!(
        "\
program: stuck
usage: stuck [subcommands]
subcommands:
        [source_file]       :   interprets the file.
        help                :   prints this page./
    "
    );
    if let Some(message) = message {
        eprintln!("Error: {}.", message);
        exit(1);
    }
    exit(0);
}
