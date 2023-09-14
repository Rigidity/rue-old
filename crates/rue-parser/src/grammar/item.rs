use rue_syntax::{SyntaxKind, T};

use crate::Parser;

use super::{parse_block, ty::parse_type};

pub(super) fn parse_item(p: &mut Parser) {
    match p.peek() {
        T![fn] => parse_fn_item(p),
        kind => p.error(format!("expected item, found {kind}")),
    }
}

fn parse_fn_item(p: &mut Parser) {
    p.start(SyntaxKind::FnItem);
    p.eat(T![fn]);
    p.eat(SyntaxKind::Ident);
    parse_fn_param_list(p);
    p.eat(T![->]);
    parse_type(p);
    parse_block(p);
    p.finish();
}

fn parse_fn_param_list(p: &mut Parser) {
    p.start(SyntaxKind::FnParamList);
    p.eat(T!['(']);

    while !matches!(p.peek(), SyntaxKind::Eof | T![')']) {
        parse_fn_param(p);

        if p.peek() == T![,] {
            p.bump();
        } else {
            break;
        }
    }

    p.eat(T![')']);
    p.finish();
}

fn parse_fn_param(p: &mut Parser) {
    p.start(SyntaxKind::FnParam);
    p.eat(SyntaxKind::Ident);
    p.eat(T![:]);
    parse_type(p);
    p.finish();
}
