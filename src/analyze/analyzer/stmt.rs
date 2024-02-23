use crate::{
    analyze::{expr::Expr, stmt::Stmt},
    file::position::Position,
    parse::{
        row_expr::RowExpr,
        row_stmt::{RowStmt, RowStmtKind},
    },
};

use super::Analyzer;

impl Analyzer {
    pub fn analyze_stmt(&mut self, row_stmt: RowStmt) -> anyhow::Result<Stmt> {
        let position = row_stmt.position;
        Ok(match row_stmt.kind {
            RowStmtKind::Expr { expr } => self.analyze_stmt_expr(expr, position)?,
            RowStmtKind::Return { expr } => self.analyze_stmt_return(expr, position)?,
            RowStmtKind::If {
                condition_expr,
                then_stmt,
                else_stmt,
            } => self.analyze_stmt_if(condition_expr, *then_stmt, *else_stmt, position)?,
            RowStmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt,
            } => {
                self.analyze_stmt_for(init_expr, condition_expr, delta_expr, *run_stmt, position)?
            }
            RowStmtKind::While {
                condition_expr,
                run_stmt,
            } => self.analyze_stmt_while(condition_expr, *run_stmt, position)?,
            RowStmtKind::Cpd { stmts } => self.analyze_stmt_cpd(stmts, position)?,
        })
    }

    fn analyze_stmt_expr(
        &mut self,
        row_expr: Option<RowExpr>,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let expr = if let Some(row_expr) = row_expr {
            Some(self.analyze_expr(row_expr)?)
        } else {
            None
        };
        Ok(Stmt::new_expr(expr, position))
    }

    fn analyze_stmt_return(
        &mut self,
        row_expr: Option<RowExpr>,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let expr = if let Some(row_expr) = row_expr {
            Some(self.analyze_expr(row_expr)?)
        } else {
            None
        };
        Ok(Stmt::new_return(expr, position))
    }

    fn analyze_stmt_if(
        &mut self,
        row_condition_expr: RowExpr,
        row_then_stmt: RowStmt,
        row_else_stmt: Option<RowStmt>,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let condition_expr = self.analyze_expr(row_condition_expr)?;
        let then_stmt = self.analyze_stmt(row_then_stmt)?;
        let else_stmt = if let Some(row_else_stmt) = row_else_stmt {
            Some(self.analyze_stmt(row_else_stmt)?)
        } else {
            None
        };
        Ok(Stmt::new_if(condition_expr, then_stmt, else_stmt, position))
    }

    fn analyze_stmt_for(
        &mut self,
        row_init_expr: Option<RowExpr>,
        row_condition_expr: Option<RowExpr>,
        row_delta_expr: Option<RowExpr>,
        row_run_stmt: RowStmt,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let init_expr = if let Some(row_init_expr) = row_init_expr {
            Some(self.analyze_expr(row_init_expr)?)
        } else {
            None
        };
        let condition_expr = if let Some(row_condition_expr) = row_condition_expr {
            self.analyze_expr(row_condition_expr)?
        } else {
            Expr::new_number(1, position.clone())
        };
        let delta_expr = if let Some(row_delta_expr) = row_delta_expr {
            Some(self.analyze_expr(row_delta_expr)?)
        } else {
            None
        };
        let run_stmt = self.analyze_stmt(row_run_stmt)?;
        Ok(Stmt::new_for(
            init_expr,
            condition_expr,
            delta_expr,
            run_stmt,
            position,
        ))
    }

    fn analyze_stmt_while(
        &mut self,
        row_condition_expr: RowExpr,
        row_run_stmt: RowStmt,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let condition_expr = self.analyze_expr(row_condition_expr)?;
        let run_stmt = self.analyze_stmt(row_run_stmt)?;
        Ok(Stmt::new_while(condition_expr, run_stmt, position))
    }

    fn analyze_stmt_cpd(
        &mut self,
        stmts: Vec<RowStmt>,
        position: Position,
    ) -> anyhow::Result<Stmt> {
        let stmts = stmts
            .into_iter()
            .map(|stmt| self.analyze_stmt(stmt))
            .collect::<anyhow::Result<Vec<Stmt>>>()?;
        Ok(Stmt::new_cpd(stmts, position))
    }
}
