//! Abstract syntax tree data structure
use crate::front_end::token::Span;

pub struct ASTNode {
    pub span: Span,
}

impl ASTNode {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            span: Span { start, end },
        }
    }
}

trait Expression {}
// literal struct: self.value
// identifier struct: self.id
// Unary op struct: self.op, self.operand
// binary op struct: self.left, self.op, self.right
// function call struct: self.func, self.args

trait Statement {}

trait Declaration {}

trait Program {}
