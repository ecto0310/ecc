use crate::file::position::Position;

use super::row_expr::RowExpr;

#[derive(Debug, Clone)]
pub struct RowStmt {
    pub kind: RowStmtKind,
    pub position: Position,
}

impl RowStmt {
    pub fn new_expr(expr: Option<RowExpr>, position: Position) -> Self {
        Self {
            kind: RowStmtKind::Expr { expr },
            position,
        }
    }

    pub fn new_return(expr: Option<RowExpr>, position: Position) -> Self {
        Self {
            kind: RowStmtKind::Return { expr },
            position,
        }
    }

    pub fn new_if(
        condition_expr: RowExpr,
        then_stmt: RowStmt,
        else_stmt: Option<RowStmt>,
        position: Position,
    ) -> Self {
        Self {
            kind: RowStmtKind::If {
                condition_expr,
                then_stmt: Box::new(then_stmt),
                else_stmt: Box::new(else_stmt),
            },
            position,
        }
    }

    pub fn new_for(
        init_expr: Option<RowExpr>,
        condition_expr: Option<RowExpr>,
        delta_expr: Option<RowExpr>,
        run_stmt: RowStmt,
        position: Position,
    ) -> Self {
        Self {
            kind: RowStmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt: Box::new(run_stmt),
            },
            position,
        }
    }

    pub fn new_while(condition_expr: RowExpr, run_stmt: RowStmt, position: Position) -> Self {
        Self {
            kind: RowStmtKind::While {
                condition_expr,
                run_stmt: Box::new(run_stmt),
            },
            position,
        }
    }

    pub fn new_cpd(stmts: Vec<RowStmt>, position: Position) -> Self {
        Self {
            kind: RowStmtKind::Cpd { stmts },
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RowStmtKind {
    Expr {
        expr: Option<RowExpr>,
    },
    Return {
        expr: Option<RowExpr>,
    },
    If {
        condition_expr: RowExpr,
        then_stmt: Box<RowStmt>,
        else_stmt: Box<Option<RowStmt>>,
    },
    For {
        init_expr: Option<RowExpr>,
        condition_expr: Option<RowExpr>,
        delta_expr: Option<RowExpr>,
        run_stmt: Box<RowStmt>,
    },
    While {
        condition_expr: RowExpr,
        run_stmt: Box<RowStmt>,
    },
    Cpd {
        stmts: Vec<RowStmt>,
    },
}
