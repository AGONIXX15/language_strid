use crate::nodes::exprs::{BinaryExpr, BinaryOp};
use crate::nodes::exprs::{LiteralExpr, LiteralValue};
use crate::parser::errors::ParserError;
use crate::parser::lookups::BindingPower;
use crate::parser::lookups::{get_led_fn, get_nud_fn, LedFn, NudFn};
use crate::{nodes::exprs::Expr, parser::engine::Parser};
use lexer::token::Token;
use lexer::token_type::TokenKind;

/// This module defines the expression parsing logic for the parser.
/// It includes the `Parser` struct and methods for parsing primary expressions and expressions
/// with different binding powers.
/// The `Parser` struct is responsible for managing the parsing state, including the current
/// token position and the sequence of tokens to be parsed.
impl<'a> Parser<'a> {
    /// Parses a primary expression, which is the most basic unit of an expression.
    pub fn parse_primary_expr(p: &mut Parser<'a>) -> Result<Box<dyn Expr + 'a>, ParserError> {
        let token = p
            .get_current_token()
            .ok_or_else(|| {
                ParserError::UnexpectedToken("Expected a primary expression".to_string())
            })?
            .clone();
        match token.kind {
            TokenKind::INTEGER => {
                // Parse an integer literal
                p.advance();
                let value = token.value.parse::<i128>().map_err(|_|{
                    ParserError::InvalidExpression(format!(
                        "Failed to parse integer literal: {}",
                        token.value
                    ))
                });
                let span = token.span.clone();
                Ok(Box::new(LiteralExpr::new(LiteralValue::Int(value?), span)))
            }
            _ => {
                // Handle other primary expression types, such as identifiers, strings, etc.
                // For now, we will just return an error for unsupported token kinds
                todo!("Handle other primary expression types");
                Err(ParserError::UnexpectedToken(format!(
                    "Unsupported primary expression token: {:?}",
                    token.kind
                )))
            }
        }
    }

    pub fn parse_expr(p: &mut Parser<'a>, bp: BindingPower) -> Result<Box<dyn Expr + 'a>, ParserError> {
        // Start with a primary expression
        let mut left: Box<dyn Expr + 'a> = Self::parse_primary_expr(p)?;

        // Check for the next token to determine if we have a binary expression
        while let Some(current) = p.get_current_token() {
        let binding_power = BindingPower::from_token(current.kind);
        if (bp as u8) >= (binding_power as u8) {
            break;
        }

        let led_fn: LedFn<'a> = get_led_fn(current.kind)?;
        left = led_fn(p, binding_power, left)?;
        }
        Ok(left)
}



    /// Parses an expression, starting with a primary expression and then applying
    /// operators based on their binding power.
    /// This method handles the precedence of operators and allows for chaining of expressions.
    /// It starts with a primary expression and then looks for operators to apply,
    /// continuing until no more operators can be applied based on their binding power.
    /// until no more operators can be applied based on their binding power.
    pub fn parse_binary_expr(
        p: &mut Parser<'a>,
        bp: BindingPower,
        left: Box<dyn Expr + 'a>,
    ) -> Result<Box<dyn Expr + 'a>, ParserError> {
        // Here we would typically look for the next token and determine if it is a binary operator
        let op: Token<'a> = p
            .get_current_token()
            .ok_or_else(|| ParserError::UnexpectedToken("Expected a binary operator".to_string()))?
            .clone();
        // Advance the parser to the next tokens
        p.advance();
        let right_bp = BindingPower::from_token(op.kind);
        let right = Self::parse_expr(p, right_bp)?;
        // Create a binary expression with the left operand, operator, and right operand
        Ok(Box::new(BinaryExpr {
            left,
            op: BinaryOp::from_token(op.kind),
            right,
            type_info: None, // Type information can be set later
        }))
    }
}
