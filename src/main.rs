mod interpreter;
mod object;
mod operation;
mod tokenizer;

use std::{env::args, process::exit};

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
                    interpreter.run(source_path);
                }
            };
        }
        3 => {
            let source_path = &args[1];
            let flag = &args[2];
            match flag.as_str() {
                "i" => {
                    let mut interpreter = Interpreter::new();
                    interpreter.run(source_path);
                }
                unknown => {
                    help(Some(&format!("Unknown flag `{}`", unknown)));
                }
            }
        }
        _ => {
            help(Some("No subcommand provided"));
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
        [source_file] i     :   interprets the file.
        help                :   prints this page./
    "
    );
    if let Some(message) = message {
        eprintln!("ERROR: {}.", message);
        exit(1);
    }
    exit(0);
}
