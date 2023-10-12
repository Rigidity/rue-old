use rowan::Checkpoint;
use rue_syntax::{SyntaxKind, T};

use crate::{grammar::parse_block, parser::Parser};

pub(super) fn parse_expr(p: &mut Parser) {
    parse_binary_expr(p, 0);
}

fn parse_binary_expr(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    if p.at_set(&[SyntaxKind::Integer, SyntaxKind::String, SyntaxKind::Ident]) {
        p.start(SyntaxKind::LiteralExpr);
        p.bump();
        p.finish();
    } else if p.at(T![if]) {
        parse_if_expr(checkpoint, p);
    } else if p.at(T![-]) {
        parse_prefix_expr(checkpoint, p, 7);
    } else if p.at(T!['(']) {
        parse_group_expr(p);
    } else {
        return p.error("expected expression".to_string());
    }

    if p.at(T!['(']) {
        parse_call_expr(checkpoint, p);
    }

    loop {
        let binding = if p.at_set(&[T![<], T![>]]) {
            (1, 2)
        } else if p.at_set(&[T![+], T![-]]) {
            (3, 4)
        } else if p.at_set(&[T![*], T![/]]) {
            (5, 6)
        } else {
            break;
        };

        if binding.0 < min_binding_power {
            return;
        }

        p.bump();

        p.start_at(checkpoint, SyntaxKind::BinaryExpr);
        parse_binary_expr(p, binding.1);
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
    p.bump();
    parse_expr(p);
    p.expect(T![')']);
}

fn parse_call_expr(checkpoint: Checkpoint, p: &mut Parser) {
    p.start_at(checkpoint, SyntaxKind::CallExpr);
    p.bump();

    while !p.at_set(&[T![')'], SyntaxKind::Eof]) {
        parse_expr(p);

        if p.at(T![,]) {
            p.bump();
        } else {
            break;
        }
    }

    p.expect(T![')']);
    p.finish();
}

fn parse_if_expr(checkpoint: Checkpoint, p: &mut Parser) {
    p.start_at(checkpoint, SyntaxKind::IfExpr);
    p.bump();
    parse_expr(p);
    parse_block(p);
    p.expect(T![else]);
    parse_block(p);
    p.finish()
}
