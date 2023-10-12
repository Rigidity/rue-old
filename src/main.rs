// use std::fs;

// use anyhow::anyhow;
// use clap::Parser;
// use rowan::ast::AstNode;
// use rue_ast::Program;
// use rue_compiler::Compiler;
// use rue_error::Error;
// use rue_parser::parse_text;

/// Rue compiler.
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// The file to compile.
//     input_file: String,
// }

fn main() -> anyhow::Result<()> {
    // let args = Args::parse();

    // let source = fs::read_to_string(&args.input_file)
    //     .map_err(|_| anyhow!("unknown file {}", args.input_file))?;

    // match compile(&source) {
    //     Ok(bytes) => println!("{}", hex::encode(&bytes)),
    //     Err(errors) => eprintln!("{:?}", errors),
    // }

    Ok(())
}

// fn compile(source: &str) -> Result<Vec<u8>, Vec<Error>> {
//     let mut errors = Vec::new();

//     let (parser_errors, node) = parse_text(source);
//     errors.extend(parser_errors);

//     let Some(program) = Program::cast(node) else {
//         return Err(errors);
//     };

//     let rue_hir::Output {
//         errors: hir_errors,
//         db,
//         scope,
//     } = rue_hir::lower(program);
//     errors.extend(hir_errors);

//     let Some(scope) = scope else {
//         return Err(errors);
//     };

//     let Some(lir) = rue_lir::lower(db, scope) else {
//         return Err(errors);
//     };

//     Ok(Compiler::new().compile_to_bytes(lir))
// }
