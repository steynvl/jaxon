use std::fs;

use lexer::Lexer;
use token::Token;

mod error;
mod lexer;
mod token;

fn main() {
    let source = fs::read_to_string("samples/hello.jaxon").expect("Could not read the file.");

    let mut lexer = Lexer::new(source.as_bytes());
    let mut token: Token = Token::Eof;

    lexer.get_token(&mut token);
    println!("{:?}", token);
    assert!(token == Token::Function);

    lexer.get_token(&mut token);
    println!("{:?}", token);
    assert!(token == Token::Id(String::from("name")));

    lexer.get_token(&mut token);
    println!("{:?}", token);
    assert!(token == Token::Number(1235))
}
