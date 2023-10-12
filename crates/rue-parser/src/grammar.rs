use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use self::{
    expr::expr,
    item::item,
    stmt::{stmt, STMT_SET},
};

mod expr;
mod item;
mod path;
mod stmt;
mod ty;

pub(super) fn program(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while !p.at_eof() {
        item(p);
    }
    p.finish();
}

fn block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.expect(T!['{']);
    while p.at_set(&STMT_SET) {
        stmt(p);
    }
    expr(p);
    p.expect(T!['}']);
    p.finish();
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use rue_lexer::{Lexer, Token};

    use crate::Parser;

    use super::program;

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

    check!(check_program: parser => program(&mut parser));

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
