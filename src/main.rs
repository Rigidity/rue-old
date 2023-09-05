use ast::{FnDef, Item};
use lexer::Lexer;
use parser::Parser;
use rowan::ast::AstNode;

use crate::{ast::Program, lang::SyntaxNode};

mod ast;
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
    let node = SyntaxNode::new_root(output.green_node.clone());
    let tree = format!("{:#?}", node);
    let tree = &tree[0..(tree.len() - 1)];
    println!("{:?}", output.errors);
    println!("{}", tree);

    let ast = Program::cast(node).unwrap();
    enter_program(ast);
}

fn enter_program(program: Program) {
    println!("Entering program");

    for item in program.items() {
        enter_item(item);
    }
}

fn enter_item(item: Item) {
    println!("Entering item");

    match item {
        Item::FnDef(fn_def) => enter_fn_def(fn_def),
    }
}

fn enter_fn_def(fn_def: FnDef) {
    println!("Entering fn def");
    println!("Name = {:?}", fn_def.name());
}
