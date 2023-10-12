use rue_syntax::SyntaxKind;

use crate::parser::Parser;

pub(super) fn parse_type(p: &mut Parser) {
    if p.at(SyntaxKind::Ident) {
        p.bump();
    } else {
        p.error("expected type".to_string());
    }
}
