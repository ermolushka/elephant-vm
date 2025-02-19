mod chunk;
mod compiler;
mod scanner;
mod table;
mod value;
mod vm;
use std::io;
use std::io::Write;
use std::process::exit;

use crate::chunk::*;
use crate::scanner::*;
use crate::table::*;
use crate::value::*;
use crate::vm::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    script: Option<String>,
    #[arg(short, long)]
    repl: bool,
}

fn main() {
    let args = Args::parse();

    // init vm before doing anything else
    let mut elephant_vm = VM::init_vm();

    if let Some(script) = args.script {
        // Run the file if script path is provided
        run_file(&script, &mut elephant_vm);
    } else if args.repl {
        // Run REPL mode if --repl flag is set
        repl(&mut elephant_vm);
    } else {
        // If no arguments provided, print usage and exit
        println!("Usage: elephant [--script <path>] [--repl]");
        exit(64);
    }

    // stop vm
    elephant_vm.free_vm();
}

fn repl(vm: &mut VM) {
    loop {
        print!("<: ");
        io::stdout().flush().unwrap();
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        println!("{}", input_text);
        &vm.interpret(&input_text);
    }
}

fn run_file(file: &str, vm: &mut VM) {
    let file_content = std::fs::read_to_string(file).expect("Failed to read file");
    let result = vm.interpret(&file_content);

    match result {
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
        InterpretResult::InterpretOk => (), // Continue execution
    }
}
