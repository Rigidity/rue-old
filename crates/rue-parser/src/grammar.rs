use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use self::{
    expr::parse_expr,
    item::parse_item,
    stmt::{parse_stmt, STMT_SET},
};

mod expr;
mod item;
mod stmt;
mod ty;

pub(super) fn parse_program(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while !p.at_eof() {
        parse_item(p);
    }
    p.finish();
}

fn parse_block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.expect(T!['{']);
    while p.at_set(&STMT_SET) {
        parse_stmt(p);
    }
    parse_expr(p);
    p.expect(T!['}']);
    p.finish();
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use rue_lexer::{Lexer, Token};

    use crate::Parser;

    use super::parse_program;

    #[macro_export]
    macro_rules! check {
        ($name:ident: $parser:ident => $run:expr) => {
            pub fn $name(input: &str, expected_tree: Expect) {
                let tokens: Vec<Token> = Lexer::new(input).collect();
                let mut $parser = Parser::new(&tokens);

                $run;

                let node = $parser.output().1;
                let raw_tree = format!("{:#?}", node);
                expected_tree.assert_eq(&raw_tree[0..(raw_tree.len() - 1)]);
            }
        };
    }

    check!(check_program: parser => parse_program(&mut parser));

    #[test]
    fn parse_nothing() {
        check_program("", expect![[r#"Program@0..0"#]]);
    }

    #[test]
    fn parse_trivia() {
        check_program(
            "// Line comment\n/* Block comment */\n",
            expect![[r#"
                Program@0..36
                  LineComment@0..15 "// Line comment"
                  Whitespace@15..16 "\n"
                  BlockComment@16..35 "/* Block comment */"
                  Whitespace@35..36 "\n""#]],
        );
    }
}
