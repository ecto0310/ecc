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
        condition: Expr,
        then_stmt: Stmt,
        else_stmt: Option<Stmt>,
        position: Position,
    ) -> Self {
        Self {
            kind: StmtKind::If {
                condition,
                then_stmt: Box::new(then_stmt),
                else_stmt: Box::new(else_stmt),
            },
            position,
        }
    }
}
