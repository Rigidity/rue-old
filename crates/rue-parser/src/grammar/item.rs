use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{parse_block, ty::parse_type};

pub(super) fn parse_item(p: &mut Parser) {
    if p.at(T![fun]) {
        parse_fun_item(p);
    } else if p.at(T![use]) {
        parse_use_item(p);
    } else {
        p.error("expected item".to_string());
    }
}

fn parse_fun_item(p: &mut Parser) {
    p.start(SyntaxKind::FunctionItem);
    p.expect(T![fun]);
    p.expect(SyntaxKind::Ident);
    parse_fun_param_list(p);
    p.expect(T![->]);
    parse_type(p);
    parse_block(p);
    p.finish();
}

fn parse_use_item(p: &mut Parser) {
    p.start(SyntaxKind::UseItem);
    p.expect(T![use]);
    p.expect(T![;]);
    p.finish();
}

fn parse_fun_param_list(p: &mut Parser) {
    p.start(SyntaxKind::FunctionParamList);
    p.expect(T!['(']);

    while !p.at_set(&[T![')'], SyntaxKind::Eof]) {
        parse_fn_param(p);

        if p.at(T![,]) {
            p.bump();
        } else {
            break;
        }
    }

    p.expect(T![')']);
    p.finish();
}

fn parse_fn_param(p: &mut Parser) {
    p.start(SyntaxKind::FunctionParam);
    p.expect(SyntaxKind::Ident);
    p.expect(T![:]);
    parse_type(p);
    p.finish();
}
