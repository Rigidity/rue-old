use rue_syntax::SyntaxKind;

use crate::Parser;

use super::{parse_block, ty::parse_type};

pub(super) fn parse_item(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Fn => parse_fn_item(p),
        kind => p.error(format!("expected item, found {kind}")),
    }
}

fn parse_fn_item(p: &mut Parser) {
    p.start(SyntaxKind::FnItem);
    p.eat(SyntaxKind::Fn);
    p.eat(SyntaxKind::Ident);
    parse_fn_param_list(p);
    p.eat(SyntaxKind::Arrow);
    parse_type(p);
    parse_block(p);
    p.finish();
}

fn parse_fn_param_list(p: &mut Parser) {
    p.start(SyntaxKind::FnParamList);
    p.eat(SyntaxKind::OpenParen);

    while !matches!(p.peek(), SyntaxKind::Eof | SyntaxKind::CloseParen) {
        parse_fn_param(p);

        if p.peek() == SyntaxKind::Comma {
            p.bump();
        } else {
            break;
        }
    }

    p.eat(SyntaxKind::CloseParen);
    p.finish();
}

fn parse_fn_param(p: &mut Parser) {
    p.start(SyntaxKind::FnParam);
    p.eat(SyntaxKind::Ident);
    p.eat(SyntaxKind::Colon);
    parse_type(p);
    p.finish();
}
