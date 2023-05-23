use std::collections::VecDeque;

use super::gen_stmt::GenStmt;

#[derive(Debug)]
pub struct GenTree {
    pub stmts: VecDeque<GenStmt>,
    pub offset: usize,
}

impl GenTree {
    pub fn new(stmts: VecDeque<GenStmt>, offset: usize) -> Self {
        Self { stmts, offset }
    }
}
