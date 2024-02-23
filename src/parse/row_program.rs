use std::collections::VecDeque;

use super::row_stmt::RowStmt;

#[derive(Debug)]
pub struct RowProgram {
    pub stmts: VecDeque<RowStmt>,
}

impl RowProgram {
    pub fn new(stmts: VecDeque<RowStmt>) -> Self {
        Self { stmts }
    }
}
