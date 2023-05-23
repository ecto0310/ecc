use crate::file::position::Position;

use super::{expr::Expr, stmt_kind::StmtKind};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub position: Position,
}

impl Stmt {
    pub fn new_expr(expr: Option<Expr>, position: Position) -> Self {
        Self {
            kind: StmtKind::Expr { expr },
            position,
        }
    }

    pub fn new_return(expr: Option<Expr>, position: Position) -> Self {
        Self {
            kind: StmtKind::Return { expr },
            position,
        }
    }

    pub fn new_if(
        condition_expr: Expr,
        then_stmt: Stmt,
        else_stmt: Option<Stmt>,
        position: Position,
    ) -> Self {
        Self {
            kind: StmtKind::If {
                condition_expr,
                then_stmt: Box::new(then_stmt),
                else_stmt: Box::new(else_stmt),
            },
            position,
        }
    }

    pub fn new_for(
        init_expr: Option<Expr>,
        condition_expr: Option<Expr>,
        delta_expr: Option<Expr>,
        run_stmt: Stmt,
        position: Position,
    ) -> Self {
        Self {
            kind: StmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt: Box::new(run_stmt),
            },
            position,
        }
    }

    pub fn new_while(condition_expr: Expr, run_stmt: Stmt, position: Position) -> Self {
        Self {
            kind: StmtKind::While {
                condition_expr,
                run_stmt: Box::new(run_stmt),
            },
            position,
        }
    }

    pub fn new_cpd(stmts: Vec<Stmt>, position: Position) -> Self {
        Self {
            kind: StmtKind::Cpd { stmts },
            position,
        }
    }
}
