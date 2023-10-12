use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

pub(super) fn path(p: &mut Parser) {
    p.start(SyntaxKind::Path);
    p.expect(SyntaxKind::Ident);
    while p.at(T![::]) {
        p.expect(T![::]);
        p.expect(SyntaxKind::Ident);
    }
    p.finish();
}
