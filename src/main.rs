mod chunk;
mod compiler;
mod scanner;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use vm::*;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();
    match argc {
        1 => {
            repl();
        }
        2 => {
            run_file(&argv[1]);
        }
        _ => {
            eprintln!("Usage: rustlox [path]");
            process::exit(64);
        }
    }
    println!("num args: {}", argc);
    for arg in argv {
        println!("arg: {}", arg);
    }
}

fn repl() {
    let mut vm = Vm::new();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        vm.interpret(buffer.clone());
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Couldn't read source file");
    let result = Vm::new().interpret(source);

    match result {
        InterpretResult::CompileError => {
            process::exit(65);
        }
        InterpretResult::RuntimeError => {
            process::exit(70);
        }
        _ => {}
    }
}
