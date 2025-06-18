use crate::token_type::TokenKind;

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
    line: usize,
    column_range: (usize, usize),
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        value: &'a str,
        line: usize,
        column_range: (usize, usize),
    ) -> Token<'a> {
        Token {
            kind: (kind),
            value: (value),
            line: (line),
            column_range: (column_range),
        }
    }
}
