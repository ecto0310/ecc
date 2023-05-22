use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr { expr: Option<Expr> },
    Return { expr: Option<Expr> },
}
