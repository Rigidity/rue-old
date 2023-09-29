use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{expr::parse_expr, ty::parse_type};

pub(super) const STMT_SET: [SyntaxKind; 1] = [T![let]];

pub(super) fn parse_stmt(p: &mut Parser) {
    if p.at(T![let]) {
        parse_let_stmt(p);
    } else {
        p.error();
    }
}

fn parse_let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.expect(T![let]);
    p.expect(SyntaxKind::Ident);

    if p.at(T![:]) {
        p.bump();
        parse_type(p);
    }

    p.expect(T![=]);
    parse_expr(p);
    p.expect(T![;]);
    p.finish();
}
