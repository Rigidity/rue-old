use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{expr::parse_expr, ty::parse_type};

pub(super) fn is_stmt(p: &mut Parser) -> bool {
    matches!(p.peek(), T![let])
}

pub(super) fn parse_stmt(p: &mut Parser) {
    match p.peek() {
        T![let] => parse_let_stmt(p),
        _ => p.error(),
    }
}

fn parse_let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.expect(T![let]);
    p.expect(SyntaxKind::Ident);

    if p.peek() == T![:] {
        p.bump();
        parse_type(p);
    }

    p.expect(T![=]);
    parse_expr(p);
    p.expect(T![;]);
    p.finish();
}
