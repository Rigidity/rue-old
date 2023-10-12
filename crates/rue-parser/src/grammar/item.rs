use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{block, path::path, ty::ty};

pub(super) fn item(p: &mut Parser) {
    if p.at(T![fun]) {
        fun_item(p);
    } else if p.at(T![use]) {
        use_item(p);
    } else {
        p.error("expected item".to_string());
    }
}

fn fun_item(p: &mut Parser) {
    p.start(SyntaxKind::FunctionItem);
    p.expect(T![fun]);
    p.expect(SyntaxKind::Ident);
    param_list(p);
    p.expect(T![->]);
    ty(p);
    block(p);
    p.finish();
}

fn use_item(p: &mut Parser) {
    p.start(SyntaxKind::UseItem);
    p.expect(T![use]);
    path(p);
    p.expect(T![;]);
    p.finish();
}

fn param_list(p: &mut Parser) {
    p.start(SyntaxKind::FunctionParamList);
    p.expect(T!['(']);

    while !p.at_set(&[T![')'], SyntaxKind::Eof]) {
        param(p);

        if p.at(T![,]) {
            p.bump();
        } else {
            break;
        }
    }

    p.expect(T![')']);
    p.finish();
}

fn param(p: &mut Parser) {
    p.start(SyntaxKind::FunctionParam);
    p.expect(SyntaxKind::Ident);
    p.expect(T![:]);
    ty(p);
    p.finish();
}
