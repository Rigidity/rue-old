use rue_syntax::SyntaxKind;

use crate::Parser;

pub(super) fn parse_type(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => {
            p.bump();
        }
        kind => p.error(format!("expected type, found {kind}")),
    }
}
