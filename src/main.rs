use rowan::ast::AstNode;
use rue_compiler::enter_program;
use rue_lexer::Lexer;
use rue_parser::{Parser, Program};
use rue_syntax::SyntaxNode;

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
