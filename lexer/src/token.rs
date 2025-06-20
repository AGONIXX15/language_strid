use crate::token_type::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct SourceSpan {
    pub start: Position,
    pub end: Position,
}

impl SourceSpan {
    pub fn combine(&self, other: &SourceSpan) -> SourceSpan {
        SourceSpan {
            start: self.start,
            end: other.end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }
}

impl SourceSpan {
    pub fn new(start: Position, end: Position) -> SourceSpan {
        SourceSpan { start: start, end: end }
    }
}

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: &'a str,
    pub span: SourceSpan,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        value: &'a str,
        span: SourceSpan
    ) -> Token<'a> {
        Token {
            kind: (kind),
            value: (value),
            span: span,
        }
    }
}
