use rue_syntax::SyntaxKind;

use crate::Parser;

use super::{expr::parse_expr, ty::parse_type};

pub(super) fn is_stmt(p: &mut Parser) -> bool {
    matches!(p.peek(), SyntaxKind::Let)
}

pub(super) fn parse_stmt(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Let => parse_let_stmt(p),
        kind => p.error(format!("expected statement, found {kind}")),
    }
}

fn parse_let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.eat(SyntaxKind::Let);
    p.eat(SyntaxKind::Ident);

    if p.peek() == SyntaxKind::Colon {
        p.bump();
        parse_type(p);
    }

    p.eat(SyntaxKind::Equals);
    parse_expr(p);
    p.finish();
}
