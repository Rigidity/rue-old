use grammar::parse_program;
use parser::Parser;
use rowan::GreenNode;
use rue_error::Error;
use rue_lexer::{Lexer, Token};

mod grammar;
mod parser;

pub fn parse_text(source: &str) -> (Vec<Error>, GreenNode) {
    let tokens: Vec<Token> = Lexer::new(source).collect();
    let mut parser = Parser::new(&tokens);
    parse_program(&mut parser);
    parser.output()
}
