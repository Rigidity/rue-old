use rue_ast::Program;
use rue_hir::Database;
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

    let ast = Program::cast(node).unwrap();
    let mut db = Database::new();
    let result = db.lower_program(ast);

    dbg!(db, result);
}
