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
}
