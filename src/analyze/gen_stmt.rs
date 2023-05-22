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
}
