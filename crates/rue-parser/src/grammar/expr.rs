use rue_syntax::{SyntaxKind, T};

use crate::Parser;

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

pub(super) fn parse_expr(p: &mut Parser) {
    parse_expr_binding_power(p, 0);
}

fn parse_expr_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        SyntaxKind::Integer | SyntaxKind::String | SyntaxKind::Ident => {
            p.bump();
        }
        T![-] => {
            let op = PrefixOp::Neg;
            let right_binding_power = op.binding_power();

            p.start_at(checkpoint, SyntaxKind::PrefixExpr);
            p.bump();
            parse_expr_binding_power(p, right_binding_power);
            p.finish();
        }
        T!['('] => {
            p.bump();
            parse_expr(p);
            p.eat(T![')']);
        }
        kind => {
            p.error(format!("expected expression, found {kind}"));
        }
    }

    if p.peek() == T!['('] {
        p.start_at(checkpoint, SyntaxKind::CallExpr);
        p.bump();

        while !matches!(p.peek(), SyntaxKind::Eof | T![')']) {
            parse_expr(p);

            if p.peek() == T![,] {
                p.bump();
            } else {
                break;
            }
        }

        p.eat(T![')']);
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
