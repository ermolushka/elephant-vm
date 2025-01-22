mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;
use std::io;
use std::io::Write;
use std::process::exit;

use crate::chunk::*;
use crate::scanner::*;
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
    let script = args.script;
    // init vm before doing anything else
    let mut elephant_vm = VM::init_vm();

    if args.repl {
        repl(&mut elephant_vm);
    }
    // } else  if let Some(script) = &script {
    //     // TODO: fix this as now it process only first line
    //     lox.run_file(&script);
    // } else {
    //     lox.run_prompt();
    //     exit(64);
    // }

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
    let file_content = std::fs::read(file).expect("file not found");
    let content = String::from_utf8_lossy(&file_content).to_string();
    let result: &InterpretResult = &vm.interpret(&content);

    match result {
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
        _ => println!("unknown result"),
    }
}
