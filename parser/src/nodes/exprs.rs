use crate::nodes::types::Type;
use lexer::token::{SourceSpan, Token};
use lexer::token_type::TokenKind;
use std::fmt::Debug;

pub trait Expr: Debug {
    // start with a span method to get the source span of the expression
    fn span(&self) -> SourceSpan;
    fn type_info(&self) -> Option<Type>;
    fn set_type_info(&mut self, typ: Type);
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i128),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LiteralValue,
    pub type_info: Option<Type>,
    pub span: SourceSpan,
}

impl LiteralExpr {
    pub fn new(value: LiteralValue, span: SourceSpan) -> Self {
        LiteralExpr {
            value: value,
            span: span,
            type_info: None,
        }
    }
}


impl Expr for LiteralExpr {
    fn span(&self) -> SourceSpan {
        // Assuming the span is derived from the value, this is a placeholder
        self.span.clone()
    }

    fn type_info(&self) -> Option<Type> {
        self.type_info.clone()
    }

    fn set_type_info(&mut self, typ: Type) {
        self.type_info = Some(typ);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Aritméticos
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    // Comparación
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Lógicos (si soportas booleanos)
    And, // &&
    Or,  // ||
}

#[derive(Debug)]
pub struct BinaryExpr<'a> {
    pub left: Box<dyn Expr + 'a>,
    pub op: BinaryOp,
    pub right: Box<dyn Expr + 'a>,
    pub type_info: Option<Type>,
}

impl<'a> Expr for BinaryExpr<'a> {
    fn span(&self) -> SourceSpan {
        // Assuming the span is derived from the left and right expressions
        let left_span = self.left.span();
        let right_span = self.right.span();
        SourceSpan::combine(&left_span, &right_span)
    }

    fn type_info(&self) -> Option<Type> {
        self.type_info.clone()
    }

    fn set_type_info(&mut self, typ: Type) {
        self.type_info = Some(typ);
    }
}

impl BinaryOp {
    pub fn from_token(kind: TokenKind) -> Self {
        match kind {
            TokenKind::PLUS => BinaryOp::Add,
            TokenKind::DASH => BinaryOp::Sub,
            TokenKind::STAR => BinaryOp::Mul,
            TokenKind::SLASH => BinaryOp::Div,
            TokenKind::MODULO => BinaryOp::Mod,
            //TokenKind::EQUAL_EQUAL => BinaryOp::Equal,
            //TokenKind::BANG_EQUAL => BinaryOp::NotEqual,
            TokenKind::LESS => BinaryOp::Less,
            //TokenKind::LESS_EQUAL => BinaryOp::LessEqual,
            TokenKind::GREATER => BinaryOp::Greater,
            //TokenKind::GREATER_EQUAL => BinaryOp::GreaterEqual,
            TokenKind::AMPER => BinaryOp::And,
            TokenKind::VERTICAL_BAR => BinaryOp::Or,
            _ => panic!("Unknown binary operator"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Ref,     // &
    Deref,   // *
    Neg,     // - (números)
    Not,     // ! (booleanos)
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<dyn Expr>,
    pub type_info: Option<Type>,
}

