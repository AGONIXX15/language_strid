use crate::nodes::types::Type;
use crate::nodes::exprs::{LiteralExpr, BinaryExpr};

pub trait TypeVisitor {
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Type;
    fn visit_binary(&mut self, expr: &BinaryExpr) -> Type;
}
