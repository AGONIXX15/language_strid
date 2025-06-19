use std::collections::HashMap;

use crate::errors::LexerError;
use crate::token::Token;
use crate::token_type::TokenKind;
use std::sync::OnceLock;

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
            filename: filename,
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
        self.pos += 1;
        self.col += 1;
    }

    fn newline(&mut self) {
        self.line += 1;
        self.col = 1;
        self.curr_start_line = self.pos;
    }

    fn token_integer(&mut self) -> Option<Token<'a>> {
        let start: usize = self.pos;
        let start_col: usize = self.col;
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

        return Some(Token::new(
            TokenKind::INTEGER,
            &self.text[start..self.pos],
            self.line,
            (start_col, self.col),
        ));
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
        let start_col: usize = self.col;
        for c in self.text[start..].chars() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let val: &str = &self.text[start..self.pos];
        let kind: TokenKind = self.keyword_or_identifier(&val);

        Some(Token::new(kind, val, self.line, (start_col, self.col)))
    }

    fn token_string(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        // set the start before consume
        let start: usize = self.pos;
        // consume the '"'
        self.advance();
        let start_col: usize = self.col;

        let mut has_end: bool = false;

        let text: &str = &self.text[self.pos..];
        for c in text.chars() {
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
                context: (&self.text[self.curr_start_line..self.pos]),
                filename: (&self.filename),
                line: (self.line),
                col: (start_col),
            });
        }
        Ok(Token::new(
            TokenKind::LITERALSTRING,
            &self.text[start..self.pos],
            self.line,
            (start_col, self.col),
        ))
    }

    fn token_string_multi(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        return Err(LexerError::UnexpectedEOF {
            context: "",
            filename: self.filename,
            line: self.line,
            col: self.col,
        });
    }

    fn token_symbols(&mut self) -> Option<Token<'a>> {
        let start: usize = self.pos;
        let c: char = self.peek()?;

        let kind: TokenKind = get_symbols().get(&c)?.clone();
        self.advance();

        Some(Token::new(
            kind,
            &self.text[start..self.pos],
            self.line,
            (self.col - 1, self.col),
        ))
    }

    // process trash(whitespaces and \n)
    fn trash(&mut self) -> Option<Token<'a>> {
        let text: &str = &self.text[self.pos..];
        for c in text.chars() {
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

    // tokenize the input
    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError<'a>> {
        let mut vector: Vec<Token> = Vec::new();
        let len_text: usize = self.text.len();
        while self.pos != len_text {
            let token: Option<Token<'a>> = match self.peek() {
                Some(c) if c.is_numeric() => self.token_integer(),
                Some(c) if c.is_alphabetic() => self.token_identifier(self.pos),
                Some(c) if c == '"' => {
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
                    let line: usize = self.line;
                    println!("{}", self.col);
                    return Err(LexerError::InvalidCharacter {
                        context: &self.text[self.curr_start_line..self.pos + 1],
                        filename: self.filename,
                        character: invalid_char,
                        line: self.line,
                        col: self.col,
                    });
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
