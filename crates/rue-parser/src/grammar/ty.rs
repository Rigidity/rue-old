use rue_syntax::SyntaxKind;

use crate::parser::Parser;

use super::path::path;

pub(super) fn ty(p: &mut Parser) {
    if p.at(SyntaxKind::Ident) {
        path(p);
    } else {
        p.error("expected type".to_string());
    }
}
