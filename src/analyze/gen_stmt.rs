use crate::file::position::Position;

use super::{gen_expr::GenExpr, gen_stmt_kind::GenStmtKind};

#[derive(Debug, Clone)]
pub struct GenStmt {
    pub kind: GenStmtKind,
    pub position: Position,
}

impl GenStmt {
    pub fn new_expr(gen_expr: Option<GenExpr>, position: Position) -> Self {
        Self {
            kind: GenStmtKind::Expr { gen_expr },
            position,
        }
    }

    pub fn new_return(gen_expr: Option<GenExpr>, position: Position) -> Self {
        Self {
            kind: GenStmtKind::Return { gen_expr },
            position,
        }
    }

    pub fn new_if(
        condition: GenExpr,
        then_stmt: GenStmt,
        else_stmt: Option<GenStmt>,
        position: Position,
    ) -> Self {
        Self {
            kind: GenStmtKind::If {
                condition,
                then_stmt: Box::new(then_stmt),
                else_stmt: Box::new(else_stmt),
            },
            position,
        }
    }

    pub fn new_for(
        init_expr: Option<GenExpr>,
        condition_expr: GenExpr,
        delta_expr: Option<GenExpr>,
        run_stmt: GenStmt,
        position: Position,
    ) -> Self {
        Self {
            kind: GenStmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt: Box::new(run_stmt),
            },
            position,
        }
    }
}
