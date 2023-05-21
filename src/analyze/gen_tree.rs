use std::collections::VecDeque;

use super::gen_expr::GenExpr;

#[derive(Debug)]
pub struct GenTree {
    pub gen_exprs: VecDeque<GenExpr>,
    pub offset: usize,
}

impl GenTree {
    pub fn new(gen_exprs: VecDeque<GenExpr>, offset: usize) -> Self {
        Self { gen_exprs, offset }
    }
}
