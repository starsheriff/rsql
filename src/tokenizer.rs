use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(Word),
    Select,
    Equal,
    Gt,
    Lt,
    LBrace,
    RBrace,
}

#[derive(Debug, PartialEq)]
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

        while let Some(token) = match_token(&mut chars)? {
            tokens.push(token);
        }

        return Ok(tokens)
    }
}

fn match_token(b: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, Error> {
    let consuming_return  = |b: &mut Peekable<Chars<'_>>, t: Token| -> Result<Option<Token>, Error> { 
        b.next();
        Ok(Some(t)) 
    };

    match b.peek() {
        Some(&ch) => match ch {
            '=' => {
                consuming_return(b, Token::Equal)
            },
            '>' => {
                consuming_return(b, Token::Gt)
            },
            '<' => {
                consuming_return(b, Token::Lt)
            },
            '{' => consuming_return(b, Token::LBrace),
            _ => {
                b.next();
                Err(Error::NotImplemented)
            },
        },
        None => Ok(None),
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

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn a_test() {
        let tokens = "=".to_string().tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Equal]);
    }

    #[test]
    fn multiple_equals() {
        let tokens = "==".to_string().tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Equal, Token::Equal]);
    }

    #[test]
    fn multiple_equals_w_whitespace() {
        let tokens = ">".to_string().tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Gt]);
    }

    #[test]
    fn single_lt() {
        let tokens = "<".to_string().tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Lt]);
    }
}
