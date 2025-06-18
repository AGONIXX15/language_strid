use crate::errors::LexerError;
use crate::token::Token;
use crate::token_type::TokenKind;

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            text,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.text[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.col += 1;
    }

    fn token_integer(&mut self, start: usize) -> Option<Token<'a>> {
        let mut end: usize = start;
        let mut start_col: usize = self.col;
        let mut end_col: usize = self.col;
        for c in self.text[start..].chars() {
            if c.is_ascii_digit() {
                self.advance();
                end += 1;
                end_col += 1;
            } else {
                break;
            }
        }

        if start == end {
            return None;
        }

        return Some(Token::new(
            TokenKind::INTEGER,
            &self.text[start..end],
            self.line,
            (start_col, end_col),
        ));
    }

    fn token_identifier(&mut self, start: usize) -> Option<Token<'a>> {
        let start_col: usize = self.col;
        let mut end_col: usize = self.col;
        let mut end: usize = start;
        for c in self.text[start..].chars() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
                end += 1;
                end_col += 1;
            } else {
                break;
            }
        }

        Some(Token::new(
            TokenKind::IDENTIFIER,
            &self.text[start..end],
            self.line,
            (start_col, end_col),
        ))
    }

    fn operator(&mut self) -> Option<Token<'a>> {
        let start: usize = self.pos;
        let c: char = match self.peek() {
            Some(ch) => ch,
            None => return None,
        };

        self.advance();
        let kind: TokenKind = match c {
            '+' => TokenKind::PLUS,
            '*' => TokenKind::STAR,
            '=' => TokenKind::EQUAL,
            '-' => TokenKind::DASH,
            '/' => TokenKind::SLASH,
            '%' => TokenKind::MODULO,
            _ => return None,
        };

        Some(Token::new(
            kind,
            &self.text[start..self.pos],
            self.line,
            (start, self.col - 1),
        ))
    }

    fn trash(&mut self) -> Option<Token<'a>> {
        let text: &str = &self.text[self.pos..];
        for c in text.chars() {
            if c == ' ' {
                self.advance();
                continue;
            } else if c == '\n' {
                self.advance();
                self.line += 1;
                self.col = 1;
            } else {
                break;
            }
        }
        None
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError> {
        let mut vector: Vec<Token> = Vec::new();
        let len_text: usize = self.text.len();
        while self.pos != len_text {
            let token: Option<Token<'a>> = match self.peek() {
                Some('0'..='9') => self.token_integer(self.pos),
                Some('a'..='z' | 'A'..='Z') => self.token_identifier(self.pos),
                Some('+' | '-' | '/' | '*' | '=' | '%' | '&' | '|') => self.operator(),
                Some(' ' | '\n') => self.trash(),
                None => None,
                Some(invalid_char) => {
                    let line: usize = self.line;
                    return Err(LexerError::InvalidCharacter(invalid_char, line, self.pos));
                }
            };
            match token {
                Some(v) => vector.push(v),
                None => continue,
            }
        }
        Ok(vector)
    }
}
