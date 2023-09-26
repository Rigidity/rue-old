use clvmr::{
    reduction::Reduction,
    serde::{node_from_bytes, node_to_bytes},
    Allocator, ChiaDialect,
};
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
    let value = lowerer.lower_program(program).unwrap();

    println!("{:?}", value);
    println!("Compiler errors: {:?}\n", lowerer.errors());

    let bytes = Compiler::new().compile_to_bytes(value);

    println!("Compiled output: {}", hex::encode(&bytes));

    let mut a = Allocator::new();
    let ptr = node_from_bytes(&mut a, &bytes).unwrap();

    let dialect = ChiaDialect::new(0);

    let nil = a.null();
    match clvmr::run_program(&mut a, &dialect, ptr, nil, u64::MAX) {
        Ok(Reduction(cost, result)) => println!(
            "Result is {} with cost {cost}",
            hex::encode(node_to_bytes(&a, result).unwrap())
        ),
        Err(error) => println!("{error}"),
    }
}
