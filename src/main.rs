use std::io;
use std::io::prelude::*;

fn main() {
    print_welcome();

    let mut input_buffer = String::new();

    loop{
        print_prompt();

        io::stdin().read_line(&mut input_buffer).expect("Could not read line.");

        match input_buffer.trim()  {
            ".q" => {
                println!("Goodbye!");
                return;
            }
            ".h" => println!("not implemented, use \".q\" to exit"),
            _ => (),
        }

        input_buffer.clear();
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
