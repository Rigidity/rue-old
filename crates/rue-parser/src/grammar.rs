use rue_syntax::SyntaxKind;

use crate::Parser;

pub(super) fn parse_program(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while is_item(p) {
        parse_item(p);
    }
    parse_expr(p);
    p.finish();
}

fn parse_block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.eat(SyntaxKind::OpenBrace);
    while is_item(p) {
        parse_item(p);
    }
    parse_expr(p);
    p.eat(SyntaxKind::CloseBrace);
    p.finish();
}

fn is_item(p: &mut Parser) -> bool {
    matches!(p.peek(), SyntaxKind::Fn)
}

fn parse_item(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Fn => parse_fn_def(p),
        kind => p.error(format!("expected item, found {}", kind)),
    }
}

fn parse_fn_def(p: &mut Parser) {
    p.start(SyntaxKind::FnDef);
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

fn parse_expr(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Integer | SyntaxKind::String | SyntaxKind::Ident => {
            p.bump();
        }
        kind => p.error(format!("expected expression, found {}", kind)),
    }
}

fn parse_type(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => {
            p.bump();
        }
        kind => p.error(format!("expected type, found {}", kind)),
    }
}
