use rowan::ast::AstNode;
use rue_ast::Program;
use rue_parser::parse_text;

fn main() {
    let mut all_errors = Vec::new();

    let source = include_str!("../main.rue");

    let (errors, output) = parse_text(source);
    all_errors.extend(errors);

    let ast = Program::cast(output);

    println!("{:?}", all_errors);
}
