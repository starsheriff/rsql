use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Word(Word),
}

pub struct Word {
}

#[derive(Debug)]
pub enum Command {
    Quit,
    Help,
}

#[derive(Debug)]
pub enum Error {
    UnknownError,
    NotImplemented,
    UnknownCommand,
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>, Error>;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Result<Vec<Token>, Error> {
        let mut chars = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        Err(Error::NotImplemented)
    }
}

pub fn match_command(input_buffer: &mut str) -> Result<Command, Error> {
    let mut chars = input_buffer.chars().peekable();

    match chars.peek() {
        Some('.') => {
            let mut buffer = String::new();
            chars.next();

            loop {
                match chars.peek() {
                    Some(&c) if c.is_alphabetic() => {
                        buffer.push(chars.next().unwrap());
                    }
                    Some(_) => break,
                    None => break,
                }
            }

            match buffer.to_lowercase().as_ref() {
                "q"  => Ok(Command::Quit),
                "h" => Ok(Command::Help),
                _ => Err(Error::UnknownCommand),
            }
        }
        _ => Err(Error::UnknownCommand),
    }
}
