use super::{expr::Expr, stmt::Stmt};

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr {
        expr: Option<Expr>,
    },
    Return {
        expr: Option<Expr>,
    },
    If {
        condition: Expr,
        then_stmt: Box<Stmt>,
        else_stmt: Box<Option<Stmt>>,
    },
}
