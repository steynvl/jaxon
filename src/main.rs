use std::fs;

use lexer::Lexer;
use token::Token;

mod error;
mod lexer;
mod token;

fn main() {
    let source = fs::read_to_string("samples/hello.jaxon").expect("Could not read the file.");

    let mut lexer = Lexer::new(source.as_bytes());
    let mut token: Token = Token::Array;
    lexer.get_token(&mut token);
    assert!(token == Token::Id(String::from("word")));
}
