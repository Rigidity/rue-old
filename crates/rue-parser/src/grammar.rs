use rue_syntax::SyntaxKind;

use crate::Parser;

use self::{
    expr::parse_expr,
    item::parse_item,
    stmt::{is_stmt, parse_stmt},
};

mod expr;
mod item;
mod stmt;
mod ty;

pub(super) fn parse_root(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while p.peek() != SyntaxKind::Eof {
        parse_item(p);
    }
    p.finish();
}

fn parse_block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.eat(SyntaxKind::OpenBrace);
    while is_stmt(p) {
        parse_stmt(p);
    }
    parse_expr(p);
    p.eat(SyntaxKind::CloseBrace);
    p.finish();
}
