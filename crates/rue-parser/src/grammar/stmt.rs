use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{expr::expr, ty::ty};

pub(super) const STMT_SET: [SyntaxKind; 1] = [T![let]];

pub(super) fn stmt(p: &mut Parser) {
    if p.at(T![let]) {
        let_stmt(p);
    } else {
        p.error("expected statement".to_string());
    }
}

fn let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.expect(T![let]);
    p.expect(SyntaxKind::Ident);

    if p.at(T![:]) {
        p.bump();
        ty(p);
    }

    p.expect(T![=]);
    expr(p);
    p.expect(T![;]);
    p.finish();
}
