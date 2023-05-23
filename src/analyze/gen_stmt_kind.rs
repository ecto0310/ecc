use super::{gen_expr::GenExpr, gen_stmt::GenStmt};

#[derive(Debug, Clone)]
pub enum GenStmtKind {
    Expr {
        expr: Option<GenExpr>,
    },
    Return {
        expr: Option<GenExpr>,
    },
    If {
        condition_expr: GenExpr,
        then_stmt: Box<GenStmt>,
        else_stmt: Box<Option<GenStmt>>,
    },
    For {
        init_expr: Option<GenExpr>,
        condition_expr: GenExpr,
        delta_expr: Option<GenExpr>,
        run_stmt: Box<GenStmt>,
    },
    While {
        condition_expr: GenExpr,
        run_stmt: Box<GenStmt>,
    },
    Cpd {
        stmts: Vec<GenStmt>,
    },
}
