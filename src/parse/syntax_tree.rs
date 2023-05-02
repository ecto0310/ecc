use std::collections::VecDeque;

use super::expr::Expr;

#[derive(Debug)]
pub struct SyntaxTree {
    exprs: VecDeque<Expr>,
}

impl SyntaxTree {
    pub fn new(exprs: VecDeque<Expr>) -> Self {
        Self { exprs }
    }
}
