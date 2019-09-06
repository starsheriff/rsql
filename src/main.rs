use std::io;
use std::io::prelude::*;
use std::process::exit;

mod tokenizer;

use tokenizer::Command;

fn main() {
    print_welcome();

    let mut input_buffer = String::new();

    loop {
        print_prompt();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Could not read line.");

        if let Ok(cmd) = tokenizer::match_command(&mut input_buffer) {
            do_command(cmd);
        }

        input_buffer.clear();
    }
}

#[derive(Debug)]
enum CommandError {
    CommandNotFound,
    NotImplemented,
    UnknownCommand,
}

fn do_command(cmd: Command) -> Result<(), CommandError> {
    match cmd {
        Command::Quit => {
            println!("Goodbye!");
            exit(0);
        }
        Command::Help => {
            println!("not implemented, use \".q\" to exit");
            Err(CommandError::NotImplemented)
        }
    }
}

fn print_welcome() {
    println!("Welcome to rsql!");
    println!("//\\\\ .h for help, .q to quit");
}

fn print_prompt() {
    print!("rsql > ");
    let _ = io::stdout().flush();
}
