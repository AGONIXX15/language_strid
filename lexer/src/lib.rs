use self::errors::LexerError;
use self::token::Token;
use self::engine::Lexer;


pub mod engine;
pub mod token;
mod token_type;
pub mod errors;




pub fn tokenize<'a>(text: &'a str, file: &'a str) -> Result<Vec<Token<'a>>, LexerError<'a>>{
    let mut lex: Lexer<'a> = Lexer::new(text, file);
    lex.tokenize()
}
