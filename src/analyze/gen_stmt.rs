use crate::file::position::Position;

use super::{gen_expr::GenExpr, gen_stmt_kind::GenStmtKind};

#[derive(Debug, Clone)]
pub struct GenStmt {
    pub kind: GenStmtKind,
    pub position: Position,
}

impl GenStmt {
    pub fn new_expr(expr: Option<GenExpr>, position: Position) -> Self {
        Self {
            kind: GenStmtKind::Expr { expr },
            position,
        }
    }

    pub fn new_return(expr: Option<GenExpr>, position: Position) -> Self {
        Self {
            kind: GenStmtKind::Return { expr },
            position,
        }
    }
}
