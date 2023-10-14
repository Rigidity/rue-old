use lir::Lir;
use rowan::ast::AstNode;
use rue_ast::Program;
use rue_error::{Error, TextRange};
use symbol::Symbol;

use crate::database::Database;

mod database;
mod hir;
mod lir;
mod scope;
mod symbol;

pub fn compile(program: Program) -> Result<Vec<u8>, Vec<Error>> {
    let end = program.syntax().text_range().end();

    let mut db = Database::new();

    let Some(scope) = hir::Lowerer::new(&mut db).lower_program(program) else {
        return Err(db.errors());
    };

    let main = scope.resolve("main").map(|main| db.symbol(main));

    let Some(Symbol::Function { body, scope }) = main else {
        db.error(Error::new(
            "missing `main` function".to_string(),
            TextRange::new(end.into(), end.into()),
        ));
        return Err(db.errors());
    };

    let lir = lir::Lowerer::new(&db).lower_hir(body.as_ref().unwrap(), scope);
    let bytes = codegen(lir);

    Ok(bytes)
}

fn codegen(lir: Lir) -> Vec<u8> {
    Vec::new()
}
