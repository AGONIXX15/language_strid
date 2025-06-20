use lexer::token::Token;

/// A parser for a sequence of tokens, allowing navigation and retrieval of tokens.
/// The parser maintains a position within the token sequence and provides methods to access
/// the current token, advance to the next token, and retrieve tokens at specific offsets.
/// The `Parser` struct is generic over a lifetime `'a`, which allows it to hold references to
/// tokens that have the same lifetime as the parser itself.
/// #Example
/// ```
/// use lexer::token::Token;
/// use parser::engine::Parser;
/// let tokens = vec![Token::new("token1"), Token::new("token2")];
/// let mut parser = Parser::new(tokens, 0);
/// assert_eq!(parser.get_current_token().unwrap().value, "token1");
/// parser.advance();
/// assert_eq!(parser.get_current_token().unwrap().value, "token2");
/// ```
pub struct Parser<'a> {
    pub tokens: &'a [Token<'a>],
    pub pos: usize,
}

/// Implementation of the `Parser` struct, providing methods to create a new parser,
/// retrieve the current token, get a token at a specific offset, and advance the parser's
/// position.
//// The `Parser` struct is designed to work with a sequence of tokens, allowing for parsing
///operations and navigation through the tokens. It provides a simple interface to access tokens
///and manage the parser's position within the token stream.
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>], pos: usize) -> Parser<'a> {
        Parser { tokens, pos }
    }

    /// Creates a new `Parser` instance with the given tokens and initial position.
    pub fn get_current_token(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    /// Retrieves the token at the specified offset from the current position.
    pub fn get_k_token(&self, offset: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.pos + offset)
    }
    
    /// Advances the parser's position by one token, moving to the next token in the sequence.
    pub fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
}
