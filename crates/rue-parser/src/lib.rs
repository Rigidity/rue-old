use grammar::parse_program;
use parser::Parser;
use rue_error::Error;
use rue_lexer::{Lexer, Token};
use rue_syntax::SyntaxNode;

mod grammar;
mod parser;

pub fn parse_text(source: &str) -> (Vec<Error>, SyntaxNode) {
    let tokens: Vec<Token> = Lexer::new(source).collect();
    let mut parser = Parser::new(&tokens);
    parse_program(&mut parser);
    parser.output()
}
