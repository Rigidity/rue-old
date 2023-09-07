use rue_ast::Program;
use rue_compiler::Compiler;
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
    println!("Parse errors: {:?}\n", output.errors);
    println!("Parse tree: {}\n", tree);

    let program = Program::cast(node);
    let compiler = Compiler::new();
    let output = compiler.compile(program.unwrap());

    println!("Compile errors: {:?}\n", output.1);
    println!("{}", hex::encode(output.0));
}
