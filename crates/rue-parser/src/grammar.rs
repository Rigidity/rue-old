use rue_syntax::SyntaxKind;

use crate::Parser;

pub(super) fn parse_root(p: &mut Parser) {
    p.start(SyntaxKind::Program);
    while p.peek() != SyntaxKind::Eof {
        parse_item(p);
    }
    p.finish();
}

fn parse_block(p: &mut Parser) {
    p.start(SyntaxKind::Block);
    p.eat(SyntaxKind::OpenBrace);
    while is_stmt(p) {
        parse_stmt(p);
    }
    parse_expr(p);
    p.eat(SyntaxKind::CloseBrace);
    p.finish();
}

fn is_stmt(p: &mut Parser) -> bool {
    matches!(p.peek(), SyntaxKind::Let)
}

fn parse_stmt(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Let => parse_let_stmt(p),
        kind => p.error(format!("expected statement, found {kind}")),
    }
}

fn parse_let_stmt(p: &mut Parser) {
    p.start(SyntaxKind::LetStmt);
    p.eat(SyntaxKind::Let);
    p.eat(SyntaxKind::Ident);

    if p.peek() == SyntaxKind::Colon {
        p.bump();
        parse_type(p);
    }

    p.eat(SyntaxKind::Equals);
    parse_expr(p);
    p.finish();
}

fn is_item(p: &mut Parser) -> bool {
    matches!(p.peek(), SyntaxKind::Fn)
}

fn parse_item(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Fn => parse_fn_item(p),
        kind => p.error(format!("expected item, found {kind}")),
    }
}

fn parse_fn_item(p: &mut Parser) {
    p.start(SyntaxKind::FnItem);
    p.eat(SyntaxKind::Fn);
    p.eat(SyntaxKind::Ident);
    parse_fn_param_list(p);
    p.eat(SyntaxKind::Arrow);
    parse_type(p);
    parse_block(p);
    p.finish();
}

fn parse_fn_param_list(p: &mut Parser) {
    p.start(SyntaxKind::FnParamList);
    p.eat(SyntaxKind::OpenParen);

    while !matches!(p.peek(), SyntaxKind::Eof | SyntaxKind::CloseParen) {
        parse_fn_param(p);

        if p.peek() == SyntaxKind::Comma {
            p.bump();
        } else {
            break;
        }
    }

    p.eat(SyntaxKind::CloseParen);
    p.finish();
}

fn parse_fn_param(p: &mut Parser) {
    p.start(SyntaxKind::FnParam);
    p.eat(SyntaxKind::Ident);
    p.eat(SyntaxKind::Colon);
    parse_type(p);
    p.finish();
}

fn parse_type(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => {
            p.bump();
        }
        kind => p.error(format!("expected type, found {kind}")),
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> u8 {
        match self {
            Self::Neg => 5,
        }
    }
}

fn parse_expr(p: &mut Parser) {
    parse_expr_binding_power(p, 0);
}

fn parse_expr_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        SyntaxKind::Integer | SyntaxKind::String | SyntaxKind::Ident => {
            p.bump();
        }
        SyntaxKind::Minus => {
            let op = PrefixOp::Neg;
            let right_binding_power = op.binding_power();

            p.start_at(checkpoint, SyntaxKind::PrefixExpr);
            p.bump();
            parse_expr_binding_power(p, right_binding_power);
            p.finish();
        }
        SyntaxKind::OpenParen => {
            p.bump();
            parse_expr(p);
            p.eat(SyntaxKind::CloseParen);
        }
        kind => {
            p.error(format!("expected expression, found {kind}"));
        }
    }

    if p.peek() == SyntaxKind::OpenParen {
        p.start_at(checkpoint, SyntaxKind::CallExpr);
        p.bump();

        while !matches!(p.peek(), SyntaxKind::Eof | SyntaxKind::CloseParen) {
            parse_expr(p);

            if p.peek() == SyntaxKind::Comma {
                p.bump();
            } else {
                break;
            }
        }

        p.eat(SyntaxKind::CloseParen);
        p.finish();
    }

    loop {
        let op = match p.peek() {
            SyntaxKind::Plus => InfixOp::Add,
            SyntaxKind::Minus => InfixOp::Sub,
            SyntaxKind::Star => InfixOp::Mul,
            SyntaxKind::Slash => InfixOp::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < min_binding_power {
            return;
        }

        p.bump();

        p.start_at(checkpoint, SyntaxKind::BinaryExpr);
        parse_expr_binding_power(p, right_binding_power);
        p.finish();
    }
}
