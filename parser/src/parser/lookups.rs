
use lexer::token_type::TokenKind;
use crate::nodes::exprs::Expr;
use crate::parser::engine::Parser;
use crate::parser::errors::{LookUpError, ParserError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingPower {
    Primary,
    Mul,
    Div,
    Add,
    Sub,

    Or,
    And,
}

impl BindingPower {
    pub fn from_token(kind: TokenKind) -> Self {
        match kind {
            TokenKind::PLUS | TokenKind::DASH => BindingPower::Add,
            TokenKind::STAR | TokenKind::SLASH | TokenKind::MODULO => BindingPower::Mul,
            TokenKind::VERTICAL_BAR => BindingPower::Or,
            TokenKind::AMPER => BindingPower::And,
            _ => BindingPower::Primary,
        }
    }
}

pub type NudFn<'a> = fn(&mut Parser<'a>) -> Result<Box<dyn Expr + 'a>, ParserError>;
pub type LedFn<'a> = fn(&mut Parser<'a>,BindingPower, Box<dyn Expr + 'a>) -> Result<Box<dyn Expr + 'a>, ParserError>;

pub fn get_nud_fn(kind: TokenKind) -> Result<NudFn<'static>, LookUpError> {
    match kind {
        TokenKind::INTEGER => Ok(Parser::parse_primary_expr),
        _ => Err(LookUpError::NotFound(format!("Nud function for token kind {:?} not found", kind))),
    }
}

pub fn get_led_fn<'a>(kind: TokenKind) -> Result<LedFn<'a>, LookUpError> {
    match kind {
        TokenKind::PLUS | TokenKind::DASH => Ok(Parser::parse_binary_expr as LedFn<'a>),
        TokenKind::STAR | TokenKind::SLASH | TokenKind::MODULO => Ok(Parser::parse_binary_expr as LedFn<'a>),
        TokenKind::VERTICAL_BAR => Ok(Parser::parse_binary_expr),
        TokenKind::AMPER => Ok(Parser::parse_binary_expr),
        _ => Err(LookUpError::NotFound(format!("Led function for token kind {:?} not found", kind))),
    }
}





