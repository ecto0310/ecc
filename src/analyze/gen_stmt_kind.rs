use super::{gen_expr::GenExpr, gen_stmt::GenStmt};

#[derive(Debug, Clone)]
pub enum GenStmtKind {
    Expr {
        gen_expr: Option<GenExpr>,
    },
    Return {
        gen_expr: Option<GenExpr>,
    },
    If {
        condition: GenExpr,
        then_stmt: Box<GenStmt>,
        else_stmt: Box<Option<GenStmt>>,
    },
    For {
        init_expr: Option<GenExpr>,
        condition_expr: GenExpr,
        delta_expr: Option<GenExpr>,
        run_stmt: Box<GenStmt>,
    },
}
