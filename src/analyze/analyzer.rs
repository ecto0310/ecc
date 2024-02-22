use std::collections::{BTreeMap, VecDeque};

use crate::{
    file::position::Position,
    parse::{
        row_expr::{RowAssignOpKind, RowBinaryOpKind, RowExpr, RowExprKind},
        row_program::RowProgram,
        row_stmt::{RowStmt, RowStmtKind},
    },
};

use super::{
    expr::{BinaryOpKind, Expr},
    program::Program,
    stmt::Stmt,
    variable::Variable,
};

pub struct Analyzer {
    var: BTreeMap<String, Variable>,
    offset: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            var: BTreeMap::new(),
            offset: 0,
        }
    }

    pub fn analyze(&mut self, row_program: RowProgram) -> anyhow::Result<Program> {
        let stmts = row_program
            .stmts
            .into_iter()
            .map(|stmt| self.analyze_stmt(stmt))
            .collect::<anyhow::Result<VecDeque<Stmt>>>()?;
        Ok(Program::new(stmts, self.offset))
    }

    fn analyze_stmt(&mut self, row_stmt: RowStmt) -> anyhow::Result<Stmt> {
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

    fn analyze_expr(&mut self, row_expr: RowExpr) -> anyhow::Result<Expr> {
        let position = row_expr.position;
        Ok(match row_expr.kind {
            RowExprKind::Binary {
                row_binary_op_kind,
                row_lhs_expr,
                row_rhs_expr,
            } => self.analyze_expr_binary(
                row_binary_op_kind,
                *row_lhs_expr,
                *row_rhs_expr,
                position,
            )?,
            RowExprKind::Assign {
                row_assign_op_kind,
                row_lhs_expr,
                row_rhs_expr,
            } => self.analyze_expr_assign(
                row_assign_op_kind,
                *row_lhs_expr,
                *row_rhs_expr,
                position,
            )?,
            RowExprKind::Comma {
                row_lhs_expr,
                row_rhs_expr,
            } => Expr::new_comma(
                self.analyze_expr(*row_lhs_expr)?,
                self.analyze_expr(*row_rhs_expr)?,
                position,
            ),
            RowExprKind::Condition {
                row_condition_expr,
                row_then_expr,
                row_else_expr,
            } => Expr::new_condition(
                self.analyze_expr(*row_condition_expr)?,
                self.analyze_expr(*row_then_expr)?,
                self.analyze_expr(*row_else_expr)?,
                position,
            ),
            RowExprKind::UnaryIncrement { row_expr } => Expr::new_assign(
                BinaryOpKind::Add,
                self.analyze_expr(*row_expr)?,
                Expr::new_number(1, position.clone()),
                position,
            ),
            RowExprKind::UnaryDecrement { row_expr } => Expr::new_assign(
                BinaryOpKind::Sub,
                self.analyze_expr(*row_expr)?,
                Expr::new_number(1, position.clone()),
                position,
            ),
            RowExprKind::PostfixIncrement { row_expr } => {
                Expr::new_postfix_increment(self.analyze_expr(*row_expr)?, position)
            }
            RowExprKind::PostfixDecrement { row_expr } => {
                Expr::new_postfix_decrement(self.analyze_expr(*row_expr)?, position)
            }
            RowExprKind::Identifier { ident } => {
                let var = self.get_var(ident);
                Expr::new_var(var, position)
            }
            RowExprKind::Number { number } => Expr::new_number(number, position),
            RowExprKind::Func {
                row_name_expr,
                row_args_expr,
            } => {
                let args = row_args_expr
                    .into_iter()
                    .map(|arg| self.analyze_expr(arg))
                    .collect::<anyhow::Result<Vec<Expr>>>()?;
                if let RowExprKind::Identifier { ident } = row_name_expr.kind {
                    Expr::new_func_label(ident, args, position)
                } else {
                    Expr::new_func_expr(self.analyze_expr(*row_name_expr)?, args, position)
                }
            }
        })
    }

    fn analyze_expr_binary(
        &mut self,
        row_binary_op_kind: RowBinaryOpKind,
        row_lhs_expr: RowExpr,
        row_rhs_expr: RowExpr,
        position: Position,
    ) -> anyhow::Result<Expr> {
        Ok(match row_binary_op_kind {
            RowBinaryOpKind::LogicAnd => Expr::new_condition(
                self.analyze_expr(row_lhs_expr)?,
                self.analyze_expr(row_rhs_expr)?,
                Expr::new_number(0, position.clone()),
                position,
            ),
            RowBinaryOpKind::LogicOr => Expr::new_condition(
                self.analyze_expr(row_lhs_expr)?,
                Expr::new_number(1, position.clone()),
                self.analyze_expr(row_rhs_expr)?,
                position,
            ),
            RowBinaryOpKind::Gt => Expr::new_binary(
                BinaryOpKind::Lt,
                self.analyze_expr(row_rhs_expr)?,
                self.analyze_expr(row_lhs_expr)?,
                position,
            ),
            RowBinaryOpKind::GtEqual => Expr::new_binary(
                BinaryOpKind::LtEqual,
                self.analyze_expr(row_rhs_expr)?,
                self.analyze_expr(row_lhs_expr)?,
                position,
            ),
            op_kind => Expr::new_binary(
                BinaryOpKind::from_row_binary_op_kind(op_kind)?,
                self.analyze_expr(row_lhs_expr)?,
                self.analyze_expr(row_rhs_expr)?,
                position,
            ),
        })
    }

    fn analyze_expr_assign(
        &mut self,
        row_assign_op_kind: RowAssignOpKind,
        row_lhs_expr: RowExpr,
        row_rhs_expr: RowExpr,
        position: Position,
    ) -> anyhow::Result<Expr> {
        let lhs = self.analyze_expr(row_lhs_expr)?;
        let rhs = self.analyze_expr(row_rhs_expr)?;
        let binary_op_kind = BinaryOpKind::from_row_assign_op_kind(row_assign_op_kind)?;
        Ok(Expr::new_assign(binary_op_kind, lhs, rhs, position))
    }

    fn get_var(&mut self, name: String) -> Variable {
        if let Some(var) = self.var.get(&name) {
            var.clone()
        } else {
            self.offset += 8;
            let var = Variable::new(self.offset);
            self.var.insert(name, var.clone());
            var
        }
    }
}
