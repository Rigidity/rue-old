use rue_ast::Program;
use rue_compiler::Compiler;
use rue_hir::Lowerer;
use rue_lexer::Lexer;
use rue_parser::Parser;
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

    println!("{}", tree);
    println!("Parse errors: {:?}\n", output.errors);

    if !output.errors.is_empty() {
        return;
    }

    let program = Program::cast(node).unwrap();
    let mut lowerer = Lowerer::new();
    let value = lowerer.lower_program(program);

    println!("{:?}", value);
    println!("Compiler errors: {:?}\n", lowerer.errors());

    println!(
        "Compiled output: {}",
        hex::encode(Compiler::new().compile_to_bytes(value.unwrap()))
    );
}
