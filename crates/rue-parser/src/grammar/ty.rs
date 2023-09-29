use rue_syntax::SyntaxKind;

use crate::parser::Parser;

pub(super) fn parse_type(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => {
            p.bump();
        }
        _ => p.error(),
    }
}
