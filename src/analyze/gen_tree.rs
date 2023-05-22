use std::collections::VecDeque;

use super::gen_stmt::GenStmt;

#[derive(Debug)]
pub struct GenTree {
    pub gen_stmts: VecDeque<GenStmt>,
    pub offset: usize,
}

impl GenTree {
    pub fn new(gen_stmts: VecDeque<GenStmt>, offset: usize) -> Self {
        Self { gen_stmts, offset }
    }
}
