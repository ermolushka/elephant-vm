use std::{env::args, process::exit};

use crate::vm::*;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;
fn main() {
    // init vm
    let mut vm: VM = VM::init_vm();
    if args().len() == 1 {
        repl();
    } else if args().len() == 2 {
        let file_path = args().nth(1).unwrap();
        run_file(&mut vm, &file_path);
    } else {
        println!("Usage: lox [script]");
        exit(64);
    }

    vm.free_vm();
}

fn repl() {
    println!("Welcome to the Lox repl!");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
    }
}

fn run_file(vm: &mut VM, file_path: &str) -> Result<(), ()> {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let result = vm.interpret(file_content.as_str());
    match result {
        InterpretResult::InterpretOk => (),
        InterpretResult::InterpretCompileError => {
            println!("Compile error");
            exit(65);
        }
        InterpretResult::InterpretRuntimeError => {
            println!("Runtime error");
            exit(70);
        }
    }
    Ok(())
}
