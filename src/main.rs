use std::fs;

use anyhow::anyhow;
use clap::Parser;
use clvmr::{
    reduction::Reduction,
    serde::{node_from_bytes, node_to_bytes},
    Allocator, ChiaDialect,
};
use rowan::ast::AstNode;
use rue_ast::Program;
use rue_compiler::Compiler;
use rue_error::Error;
use rue_parser::parse_text;

/// Rue compiler.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to compile.
    input_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source = fs::read_to_string(&args.input_file)
        .map_err(|_| anyhow!("unknown file {}", args.input_file))?;

    match compile(&source) {
        Ok(bytes) => {
            println!("{}", hex::encode(&bytes));

            let mut a = Allocator::new();
            let ptr = node_from_bytes(&mut a, &bytes).unwrap();
            let dialect = ChiaDialect::new(0);
            let nil = a.null();

            match clvmr::run_program(&mut a, &dialect, ptr, nil, u64::MAX) {
                Ok(Reduction(cost, result)) => println!(
                    "\n= {} ({cost} cost)",
                    hex::encode(node_to_bytes(&a, result).unwrap())
                ),
                Err(error) => println!("{error}"),
            }
        }
        Err(errors) => {
            eprintln!("{:?}", errors);
        }
    }

    Ok(())
}

fn compile(source: &str) -> Result<Vec<u8>, Vec<Error>> {
    let mut errors = Vec::new();

    let (parser_errors, node) = parse_text(source);
    errors.extend(parser_errors);

    let Some(program) = Program::cast(node) else {
        return Err(errors);
    };

    let rue_hir::Output {
        errors: hir_errors,
        db,
        scope: Some(scope),
    } = rue_hir::lower(program)
    else {
        return Err(errors);
    };

    errors.extend(hir_errors);

    let Some(lir) = rue_lir::lower(db, scope) else {
        return Err(errors);
    };

    Ok(Compiler::new().compile_to_bytes(lir))
}
