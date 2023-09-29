use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use super::{parse_block, ty::parse_type};

pub(super) fn parse_item(p: &mut Parser) {
    if p.at(T![fn]) {
        parse_fn_item(p);
    } else {
        p.error();
    }
}

fn parse_fn_item(p: &mut Parser) {
    p.start(SyntaxKind::FnItem);
    p.expect(T![fn]);
    p.expect(SyntaxKind::Ident);
    parse_fn_param_list(p);
    p.expect(T![->]);
    parse_type(p);
    parse_block(p);
    p.finish();
}

fn parse_fn_param_list(p: &mut Parser) {
    p.start(SyntaxKind::FnParamList);
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
    p.start(SyntaxKind::FnParam);
    p.expect(SyntaxKind::Ident);
    p.expect(T![:]);
    parse_type(p);
    p.finish();
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::grammar::tests::check_program;

    #[test]
    fn parse_fn() {
        check_program(
            "fn fourty_two(value: Int) -> Int { 42 }",
            expect![[r#"
                Program@0..39
                  FnItem@0..39
                    Fn@0..2 "fn"
                    Whitespace@2..3 " "
                    Ident@3..13 "fourty_two"
                    FnParamList@13..26
                      OpenParen@13..14 "("
                      FnParam@14..24
                        Ident@14..19 "value"
                        Colon@19..20 ":"
                        Whitespace@20..21 " "
                        Ident@21..24 "Int"
                      CloseParen@24..25 ")"
                      Whitespace@25..26 " "
                    Arrow@26..28 "->"
                    Whitespace@28..29 " "
                    Ident@29..32 "Int"
                    Block@32..39
                      Whitespace@32..33 " "
                      OpenBrace@33..34 "{"
                      Whitespace@34..35 " "
                      Integer@35..37 "42"
                      Whitespace@37..38 " "
                      CloseBrace@38..39 "}""#]],
        );
    }

    #[test]
    fn parse_incomplete_fn_keyword() {
        check_program(
            "fn",
            expect![[r#"
            Program@0..2
              FnItem@0..2
                Fn@0..2 "fn"
                FnParamList@2..2
                Block@2..2"#]],
        );
    }

    #[test]
    fn parse_incomplete_fn_name() {
        check_program(
            "fn incomplete",
            expect![[r#"
                Program@0..13
                  FnItem@0..13
                    Fn@0..2 "fn"
                    Whitespace@2..3 " "
                    Ident@3..13 "incomplete"
                    FnParamList@13..13
                    Block@13..13"#]],
        );
    }
}
