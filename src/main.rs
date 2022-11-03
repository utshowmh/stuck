mod interpreter;
mod object;
mod operation;
mod tokenizer;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => {
            let mut interpreter = Interpreter::new();
            loop {
                print!("stuck :> ");
                stdout().flush().unwrap_or_else(|err| {
                    eprintln!("Error: {:#?}", err);
                    exit(1);
                });
                let mut source = String::new();
                stdin().read_line(&mut source).unwrap_or_else(|err| {
                    eprintln!("Error: {:#?}", err);
                    exit(1);
                });
                interpreter.run(&source);
            }
        }

        2 => {
            match args[1].as_str() {
                "help" => {
                    help(None);
                }

                source_path => {
                    let mut interpreter = Interpreter::new();
                    let mut source = read_to_string(source_path).unwrap_or_else(|err| {
                        eprintln!("Error: {:#?}", err);
                        exit(2);
                    });
                    source.push('\n');
                    interpreter.run(&source);
                }
            };
        }
        3 => match args[2].as_str() {
            "-i" => {
                let mut interpreter = Interpreter::new();
                let mut source = read_to_string(&args[1]).unwrap_or_else(|err| {
                    eprintln!("Error: {:#?}", err);
                    exit(2);
                });
                source.push('\n');
                interpreter.run(&source);
            }

            "-c" => {
                todo!();
            }

            invalid_flag => help(Some(&format!("Error: invalid flag `{}`", invalid_flag))),
        },
        _ => {
            help(Some("invalid subcommands"));
        }
    }
}

fn help(message: Option<&str>) {
    println!(
        "\
program: stuck
usage: 
commands:
        stuck               :   runs a stuck repl.
        stuck [subcommands] [options]
subcommands:
        [source_file]       :   interprets the file.
        [source_file] -i    :   interprets the file.
        [source_file] -c    :   compiles the file.
        help                :   prints this page./
    "
    );
    if let Some(message) = message {
        eprintln!("Error: {}.", message);
        exit(1);
    }
    exit(0);
}
