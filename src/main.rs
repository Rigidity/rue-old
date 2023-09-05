use rowan::ast::AstNode;
use rue_lexer::Lexer;
use rue_parser::{FnDef, Item, Parser, Program};
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
