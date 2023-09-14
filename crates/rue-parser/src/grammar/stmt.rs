use rue_syntax::{SyntaxKind, T};

use crate::Parser;

use super::{expr::parse_expr, ty::parse_type};

pub(super) fn is_stmt(p: &mut Parser) -> bool {
    matches!(p.peek(), T![let])
}

pub(super) fn parse_stmt(p: &mut Parser) {
    match p.peek() {
        T![let] => parse_let_stmt(p),
        kind => p.error(format!("expected statement, found {kind}")),
    }
}

fn parse_let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.eat(T![let]);
    p.eat(SyntaxKind::Ident);

    if p.peek() == T![:] {
        p.bump();
        parse_type(p);
    }

    p.eat(T![=]);
    parse_expr(p);
    p.finish();
}
