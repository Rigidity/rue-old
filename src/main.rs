use rowan::ast::AstNode;
use rue_ast::Program;
use rue_compiler::compile;
use rue_parser::parse_text;

fn main() {
    let mut all_errors = Vec::new();

    let source = include_str!("../main.rue");

    let (errors, output) = parse_text(source);
    all_errors.extend(errors);

    if let Some(program) = Program::cast(output) {
        match compile(program) {
            Ok(bytes) => println!("{}", hex::encode(bytes)),
            Err(errors) => all_errors.extend(errors),
        }
    }

    println!("{:?}", all_errors);
}
