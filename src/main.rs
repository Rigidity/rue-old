use hir::Database;
use rue_ast as ast;
use rue_compiler::Compiler;
use rue_hir as hir;
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

    println!("{}\n", tree);
    println!("Parse errors: {:?}", output.errors);

    if !output.errors.is_empty() {
        return;
    }

    let program = ast::Program::cast(node);
    let compiler = Compiler::new();
    let output = compiler.compile(program.clone().unwrap());

    println!("{}\n", hex::encode(output.0));
    println!("Compile errors: {:?}", output.1);

    let mut db = Database::default();
    let hir = db.lower_program(program.unwrap());
    dbg!(hir);
}
