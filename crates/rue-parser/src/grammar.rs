use rue_syntax::{SyntaxKind, T};

use crate::Parser;

use self::{
    expr::parse_expr,
    item::parse_item,
    stmt::{is_stmt, parse_stmt},
};

mod expr;
mod item;
mod stmt;
mod ty;

pub(super) fn parse_root(p: &mut Parser) {
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
    use rue_lexer::{Lexer, Token};
    use rue_syntax::SyntaxNode;

    use crate::{Output, Parser};

    pub fn check(input: &str, expected_tree: Expect) {
        let tokens: Vec<Token> = Lexer::new(input).collect();
        let Output { green_node, .. } = Parser::new(&tokens).parse();
        let raw_tree = format!("{:#?}", SyntaxNode::new_root(green_node));
        expected_tree.assert_eq(&raw_tree[0..(raw_tree.len() - 1)]);
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Program@0..0"#]]);
    }
}
