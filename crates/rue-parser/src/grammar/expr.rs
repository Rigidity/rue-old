use rowan::Checkpoint;
use rue_syntax::{SyntaxKind, T};

use crate::{grammar::parse_block, Parser};

pub(super) fn parse_expr(p: &mut Parser) {
    parse_binary_expr(p, 0);
}

fn parse_binary_expr(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        SyntaxKind::Integer | SyntaxKind::String | SyntaxKind::Ident => p.bump(),
        T![if] => parse_if_expr(checkpoint, p),
        T![-] => parse_prefix_expr(checkpoint, p, 7),
        T!['('] => parse_group_expr(p),
        kind => p.error(format!("expected expression, found {kind}")),
    }

    if p.peek() == T!['('] {
        parse_call_expr(checkpoint, p);
    }

    loop {
        let (left_binding_power, right_binding_power) = match p.peek() {
            T![<] | T![>] => (1, 2),
            T![+] | T![-] => (3, 4),
            T![*] | T![/] => (5, 6),
            _ => return,
        };

        if left_binding_power < min_binding_power {
            return;
        }

        p.bump();

        p.start_at(checkpoint, SyntaxKind::BinaryExpr);
        parse_binary_expr(p, right_binding_power);
        p.finish();
    }
}

fn parse_prefix_expr(checkpoint: Checkpoint, p: &mut Parser, op_binding_power: u8) {
    p.start_at(checkpoint, SyntaxKind::PrefixExpr);
    p.bump();
    parse_binary_expr(p, op_binding_power);
    p.finish();
}

fn parse_group_expr(p: &mut Parser) {
    debug_assert!(p.peek() == T!['(']);

    p.bump();
    parse_expr(p);
    p.eat(T![')']);
}

fn parse_call_expr(checkpoint: Checkpoint, p: &mut Parser) {
    debug_assert!(p.peek() == T!['(']);

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

fn parse_if_expr(checkpoint: Checkpoint, p: &mut Parser) {
    debug_assert!(p.peek() == T![if]);

    p.start_at(checkpoint, SyntaxKind::IfExpr);
    p.bump();
    parse_expr(p);
    parse_block(p);

    if p.peek() == T![else] {
        p.bump();
        parse_block(p);
    }
}
