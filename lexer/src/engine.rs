use std::collections::HashMap;
use std::sync::OnceLock;

use crate::errors::LexerError;
use crate::token::{Position, SourceSpan, Token};
use crate::token_type::TokenKind;

static SYMBOLS: OnceLock<HashMap<char, TokenKind>> = OnceLock::new();

fn get_symbols() -> &'static HashMap<char, TokenKind> {
    SYMBOLS.get_or_init(|| {
        HashMap::from([
            ('+', TokenKind::PLUS),
            ('-', TokenKind::DASH),
            ('*', TokenKind::STAR),
            ('/', TokenKind::SLASH),
            ('=', TokenKind::EQUAL),
            ('%', TokenKind::MODULO),
            ('&', TokenKind::AMPER),
            ('|', TokenKind::VERTICAL_BAR),
            ('(', TokenKind::LPAREN),
            (')', TokenKind::RPAREN),
            ('{', TokenKind::LBRACE),
            ('}', TokenKind::RBRACE),
            ('[', TokenKind::LBRACKET),
            (']', TokenKind::RBRACKET),
            (',', TokenKind::COMMA),
            (';', TokenKind::SEMICOLON),
            (':', TokenKind::COLON),
            ('!', TokenKind::NEGATION),
            ('<', TokenKind::LESS),
            ('>', TokenKind::GREATER),
        ])
    })
}

pub struct Lexer<'a> {
    filename: &'a str,
    text: &'a str,
    curr_start_line: usize,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, filename: &'a str) -> Lexer<'a> {
        Lexer {
            filename,
            text,
            curr_start_line: 0,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.text[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        self.pos += self.peek().map_or(0, char::len_utf8);
        self.col += 1;
    }

    fn newline(&mut self) {
        self.line += 1;
        self.col = 1;
        self.curr_start_line = self.pos;
    }

    fn token_float(&mut self) -> Option<Token<'a>> {
        let start_pos = self.pos;
        let start_col = self.col;
        let mut current_pos = self.pos;
        let mut current_col = self.col;
        let mut has_dot = false;
        let mut has_digit = false;

        for c in self.text[start_pos..].chars() {
            if c.is_ascii_digit() {
                has_digit = true;
                current_pos += c.len_utf8();
                current_col += 1;
            } else if !has_dot && c == '.' {
                has_dot = true;
                current_pos += c.len_utf8();
                current_col += 1;
            } else if has_dot && c == '.' {
                return None;
            } else {
                break;
            }
        }

        if has_dot && has_digit {
            let span = SourceSpan::new(
                Position::new(self.line, start_col),
                Position::new(self.line, current_col),
            );

            let lexeme = &self.text[start_pos..current_pos];

            self.pos = current_pos;
            self.col = current_col;

            Some(Token::new(TokenKind::FLOAT, lexeme, span))
        } else {
            None
        }
    }

    fn token_integer(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let start_col = self.col;
        for c in self.text[start..].chars() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if start == self.pos {
            return None;
        }

        let span = SourceSpan::new(
            Position::new(self.line, start_col),
            Position::new(self.line, self.col),
        );

        Some(Token::new(
            TokenKind::INTEGER,
            &self.text[start..self.pos],
            span,
        ))
    }

    fn token_number(&mut self) -> Option<Token<'a>> {
        if let Some(f) = self.token_float() {
            return Some(f);
        }
        self.token_integer()
    }

    fn keyword_or_identifier(&self, word: &str) -> TokenKind {
        match word {
            "if" => TokenKind::IF,
            "else" => TokenKind::ELSE,
            "while" => TokenKind::WHILE,
            "for" => TokenKind::FOR,
            "func" => TokenKind::FUNCTION,
            "return" => TokenKind::RETURN,
            _ => TokenKind::IDENTIFIER,
        }
    }

    fn token_identifier(&mut self, start: usize) -> Option<Token<'a>> {
        let start_col = self.col;
        for c in self.text[start..].chars() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let val = &self.text[start..self.pos];
        let kind = self.keyword_or_identifier(val);
        let span = SourceSpan::new(
            Position::new(self.line, start_col),
            Position::new(self.line, self.col),
        );

        Some(Token::new(kind, val, span))
    }

    fn token_string(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        let start = self.pos;
        let start_col = self.col;
        self.advance(); // consume opening "

        let mut has_end = false;

        for c in self.text[self.pos..].chars() {
            if c == '"' {
                self.advance();
                has_end = true;
                break;
            } else if c == '\n' {
                break;
            } else {
                self.advance();
            }
        }

        if !has_end {
            return Err(LexerError::UnterminatedString {
                context: &self.text[self.curr_start_line..self.pos],
                filename: self.filename,
                line: self.line,
                col: start_col,
            });
        }

        let span = SourceSpan::new(
            Position::new(self.line, start_col),
            Position::new(self.line, self.col),
        );

        Ok(Token::new(
            TokenKind::LITERALSTRING,
            &self.text[start..self.pos],
            span,
        ))
    }

    fn token_string_multi(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        Err(LexerError::UnexpectedEOF {
            context: "",
            filename: self.filename,
            line: self.line,
            col: self.col,
        })
    }

    fn token_symbols(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let start_col = self.col;

        let c = self.peek()?;
        let kind = get_symbols().get(&c)?.clone();
        self.advance();

        let span = SourceSpan::new(
            Position::new(self.line, start_col),
            Position::new(self.line, self.col),
        );

        Some(Token::new(kind, &self.text[start..self.pos], span))
    }

    fn trash(&mut self) -> Option<Token<'a>> {
        for c in self.text[self.pos..].chars() {
            if c == ' ' {
                self.advance();
                continue;
            } else if c == '\n' {
                self.advance();
                self.newline();
            } else {
                break;
            }
        }
        None
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError<'a>> {
        let mut vector: Vec<Token> = Vec::new();
        let len_text = self.text.len();
        while self.pos != len_text {
            let token: Option<Token<'a>> = match self.peek() {
                Some(c) if c.is_numeric() || c == '.' => self.token_number(),
                Some(c) if c.is_alphabetic() => self.token_identifier(self.pos),
                Some('"') => {
                    if self.text[self.pos..].starts_with("\"\"\"") {
                        Some(self.token_string_multi()?)
                    } else {
                        Some(self.token_string()?)
                    }
                }
                Some(c) if get_symbols().contains_key(&c) => self.token_symbols(),
                Some(' ' | '\n') => self.trash(),
                None => None,
                Some(invalid_char) => {
                    return Err(LexerError::InvalidCharacter {
                        context: &self.text[self.curr_start_line..self.pos + 1],
                        filename: self.filename,
                        character: invalid_char,
                        line: self.line,
                        col: self.col,
                    });
                }
            };
            if let Some(t) = token {
                vector.push(t);
            }
        }
        Ok(vector)
    }
}
