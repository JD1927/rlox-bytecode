mod chunk;
mod compiler;
mod precedence;
mod scanner;
mod token;
mod value;
mod vm;
use std::{
    env::args,
    io::{self, Write},
    process::exit,
};

use crate::vm::{InterpretResult, VM};

fn main() {
    let args: Vec<String> = args().collect();
    let mut vm: VM = VM::new();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]).expect("Could not run file!"),
        _ => {
            eprintln!("Usage: r-lox bytecode [script]");
            exit(64);
        }
    }

    vm.free();
}

fn run_file(vm: &mut VM, path: &str) -> io::Result<()> {
    let source = std::fs::read_to_string(path)?;
    let result = vm.interpret(&source);
    match result {
        InterpretResult::CompileErrors => exit(65),
        InterpretResult::RuntimeErrors => exit(70),
        InterpretResult::Ok => exit(0),
    }
}

fn repl(vm: &mut VM) {
    let mut line = String::new();
    loop {
        print!("> ");

        if io::stdout().flush().is_err() {
            break;
        }

        line.clear();

        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                vm.interpret(&line);
            }
            Err(_) => {
                break;
            }
        }
    }
}
