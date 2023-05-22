use std::collections::VecDeque;

use super::stmt::Stmt;

#[derive(Debug)]
pub struct SyntaxTree {
    pub stmts: VecDeque<Stmt>,
}
impl SyntaxTree {
    pub fn new(stmts: VecDeque<Stmt>) -> Self {
        Self { stmts }
    }
}
