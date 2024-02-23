use std::collections::VecDeque;

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Program {
    pub stmts: VecDeque<Stmt>,
    pub offset: usize,
}

impl Program {
    pub fn new(stmts: VecDeque<Stmt>, offset: usize) -> Self {
        Self { stmts, offset }
    }
}
