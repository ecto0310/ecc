use super::gen_expr::GenExpr;

#[derive(Debug, Clone)]
pub enum GenStmtKind {
    Expr { gen_expr: Option<GenExpr> },
    Return { gen_expr: Option<GenExpr> },
}
