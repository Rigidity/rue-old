use lexer::Lexer;
use parser::Parser;

use crate::lang::SyntaxNode;

mod error;
mod lang;
mod lexer;
mod output;
mod parser;
mod syntax_kind;
mod token;
mod token_kind;

fn main() {
    let source = include_str!("../main.rue");
    let lexer = Lexer::new(source);
    let tokens = lexer.collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let output = parser.parse();
    let tree = SyntaxNode::new_root(output.green_node.clone());
    let tree = format!("{:#?}", tree);
    let tree = &tree[0..(tree.len() - 1)];
    println!("{:?}", output.errors);
    println!("{}", tree);
}
