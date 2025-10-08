use std::{env::args, process::exit};

use crate::vm::*;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;
fn main() {
    // reading --debug flag
    let args: Vec<String> = args().collect();
    let debug = args.contains(&"--debug".to_string());

    // Filter out the --debug flag to get the actual arguments
    let filtered_args: Vec<String> = args
        .iter()
        .filter(|&arg| arg != "--debug")
        .cloned()
        .collect();

    // init vm
    let mut vm: VM = VM::init_vm(Some(debug));
    if filtered_args.len() == 1 {
        repl(&mut vm);
    } else if filtered_args.len() == 2 {
        let file_path = filtered_args[1].clone();
        let _ = run_file(&mut vm, &file_path);
    } else {
        println!("Usage: elephant-vm [--debug] [script]");
        exit(64);
    }

    vm.free_vm();
}

fn repl(vm: &mut VM) {
    println!("Welcome to the Elephant VM repl!");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
        let _ = vm.interpret(input.as_str());
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
