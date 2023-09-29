use rue_syntax::{SyntaxKind, T};

use crate::parser::Parser;

use self::{
    expr::parse_expr,
    item::parse_item,
    stmt::{is_stmt, parse_stmt},
};

mod expr;
mod item;
mod stmt;
mod ty;

pub(super) fn parse_program(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while p.peek() != SyntaxKind::Eof {
        parse_item(p);
    }
    p.finish();
}

fn parse_block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.eat(T!['{']);
    while is_stmt(p) {
        parse_stmt(p);
    }
    parse_expr(p);
    p.eat(T!['}']);
    p.finish();
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use rue_error::Error;
    use rue_lexer::{Lexer, Token};
    use rue_syntax::SyntaxNode;

    use crate::Parser;

    use super::parse_program;

    #[macro_export]
    macro_rules! check {
        ($name:ident: $parser:ident => $run:expr) => {
            pub fn $name(input: &str, expected_tree: Expect, expected_errors: &[Error]) {
                let tokens: Vec<Token> = Lexer::new(input).collect();
                let mut $parser = Parser::new(&tokens);

                $run;

                let (errors, green_node) = $parser.output();

                let raw_tree = format!("{:#?}", SyntaxNode::new_root(green_node));
                expected_tree.assert_eq(&raw_tree[0..(raw_tree.len() - 1)]);

                assert_eq!(errors, expected_errors);
            }
        };
    }

    check!(check_program: parser => parse_program(&mut parser));

    #[test]
    fn parse_nothing() {
        check_program("", expect![[r#"Program@0..0"#]], &[]);
    }
}
