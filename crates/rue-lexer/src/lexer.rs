use std::str::Chars;

use crate::Token;

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            chars: source.chars(),
        }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn source(&self) -> &'a str {
        self.source
    }

    pub(crate) fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    pub(crate) fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(c) => {
                self.pos += c.len_utf8();
                c
            }
            None => '\0',
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}
