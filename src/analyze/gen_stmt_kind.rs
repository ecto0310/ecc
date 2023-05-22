use super::gen_expr::GenExpr;

#[derive(Debug, Clone)]
pub enum GenStmtKind {
    Expr { expr: Option<GenExpr> },
    Return { expr: Option<GenExpr> },
}
