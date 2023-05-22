use std::collections::{BTreeMap, VecDeque};

use crate::{
    error::Error,
    file::position::Position,
    parse::{
        expr::Expr,
        expr_kind::{AssignOpKind, BinaryOpKind, ExprKind},
        stmt::Stmt,
        stmt_kind::StmtKind,
        syntax_tree::SyntaxTree,
    },
};

use super::{
    gen_expr::GenExpr, gen_expr_kind::GenBinaryOpKind, gen_stmt::GenStmt, gen_tree::GenTree,
    var::Var,
};

pub struct Analyzer {
    var: BTreeMap<String, Var>,
    offset: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            var: BTreeMap::new(),
            offset: 0,
        }
    }

    pub fn analyze(&mut self, syntax_tree: SyntaxTree) -> Result<GenTree, Error> {
        let mut gen_stmts = VecDeque::new();
        for stmt in syntax_tree.stmts.into_iter() {
            gen_stmts.push_back(self.analyze_stmt(stmt)?)
        }
        Ok(GenTree::new(gen_stmts, self.offset))
    }

    fn analyze_stmt(&mut self, stmt: Stmt) -> Result<GenStmt, Error> {
        let position = stmt.position;
        Ok(match stmt.kind {
            StmtKind::Expr { expr } => self.analyze_stmt_expr(expr, position)?,
            StmtKind::Return { expr } => self.analyze_stmt_return(expr, position)?,
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => self.analyze_stmt_if(condition, *then_stmt, *else_stmt, position)?,
        })
    }

    fn analyze_stmt_expr(
        &mut self,
        expr: Option<Expr>,
        position: Position,
    ) -> Result<GenStmt, Error> {
        Ok(if let Some(expr) = expr {
            GenStmt::new_expr(Some(self.analyze_expr(expr)?), position)
        } else {
            GenStmt::new_expr(None, position)
        })
    }

    fn analyze_stmt_return(
        &mut self,
        expr: Option<Expr>,
        position: Position,
    ) -> Result<GenStmt, Error> {
        Ok(if let Some(expr) = expr {
            GenStmt::new_return(Some(self.analyze_expr(expr)?), position)
        } else {
            GenStmt::new_return(None, position)
        })
    }

    fn analyze_stmt_if(
        &mut self,
        condition: Expr,
        then_stmt: Stmt,
        else_stmt: Option<Stmt>,
        position: Position,
    ) -> Result<GenStmt, Error> {
        let condition = self.analyze_expr(condition)?;
        let then_stmt = self.analyze_stmt(then_stmt)?;
        let else_stmt = if let Some(else_stmt) = else_stmt {
            Some(self.analyze_stmt(else_stmt)?)
        } else {
            None
        };
        Ok(GenStmt::new_if(condition, then_stmt, else_stmt, position))
    }

    fn analyze_expr(&mut self, expr: Expr) -> Result<GenExpr, Error> {
        let position = expr.position;
        Ok(match expr.kind {
            ExprKind::Binary { op_kind, lhs, rhs } => {
                self.analyze_expr_binary(op_kind, *lhs, *rhs, position)?
            }
            ExprKind::Assign { op_kind, lhs, rhs } => {
                self.analyze_expr_assign(op_kind, *lhs, *rhs, position)?
            }
            ExprKind::Comma { lhs, rhs } => {
                GenExpr::new_comma(self.analyze_expr(*lhs)?, self.analyze_expr(*rhs)?, position)
            }
            ExprKind::Condition {
                condition,
                then_expr,
                else_expr,
            } => GenExpr::new_condition(
                self.analyze_expr(*condition)?,
                self.analyze_expr(*then_expr)?,
                self.analyze_expr(*else_expr)?,
                position,
            ),
            ExprKind::UnaryIncrement { expr } => GenExpr::new_assign_op(
                GenBinaryOpKind::Add,
                self.analyze_expr(*expr)?,
                GenExpr::new_number(1, position.clone()),
                position,
            ),
            ExprKind::UnaryDecrement { expr } => GenExpr::new_assign_op(
                GenBinaryOpKind::Sub,
                self.analyze_expr(*expr)?,
                GenExpr::new_number(1, position.clone()),
                position,
            ),
            ExprKind::PostfixIncrement { expr } => {
                GenExpr::new_postfix_increment(self.analyze_expr(*expr)?, position)
            }
            ExprKind::PostfixDecrement { expr } => {
                GenExpr::new_postfix_decrement(self.analyze_expr(*expr)?, position)
            }
            ExprKind::Identifier { name } => {
                let var = self.get_var(name);
                GenExpr::new_var(var, position)
            }
            ExprKind::Number { number } => GenExpr::new_number(number, position),
        })
    }

    fn analyze_expr_binary(
        &mut self,
        op_kind: BinaryOpKind,
        lhs: Expr,
        rhs: Expr,
        position: Position,
    ) -> Result<GenExpr, Error> {
        Ok(match op_kind {
            BinaryOpKind::LogicAnd => GenExpr::new_condition(
                self.analyze_expr(lhs)?,
                self.analyze_expr(rhs)?,
                GenExpr::new_number(0, position.clone()),
                position,
            ),
            BinaryOpKind::LogicOr => GenExpr::new_condition(
                self.analyze_expr(lhs)?,
                GenExpr::new_number(1, position.clone()),
                self.analyze_expr(rhs)?,
                position,
            ),
            BinaryOpKind::Gt => GenExpr::new_binary(
                GenBinaryOpKind::Lt,
                self.analyze_expr(rhs)?,
                self.analyze_expr(lhs)?,
                position,
            ),
            BinaryOpKind::GtEqual => GenExpr::new_binary(
                GenBinaryOpKind::LtEqual,
                self.analyze_expr(rhs)?,
                self.analyze_expr(lhs)?,
                position,
            ),
            op_kind => GenExpr::new_binary(
                op_kind.convert_to_gen()?,
                self.analyze_expr(lhs)?,
                self.analyze_expr(rhs)?,
                position,
            ),
        })
    }

    fn analyze_expr_assign(
        &mut self,
        op_kind: AssignOpKind,
        lhs: Expr,
        rhs: Expr,
        position: Position,
    ) -> Result<GenExpr, Error> {
        let lhs = self.analyze_expr(lhs)?;
        let rhs = self.analyze_expr(rhs)?;
        if op_kind == AssignOpKind::Equal {
            return Ok(GenExpr::new_assign(lhs, rhs, position));
        }
        let op_kind = op_kind.convert_to_binary()?;
        Ok(GenExpr::new_assign_op(op_kind, lhs, rhs, position))
    }

    fn get_var(&mut self, name: String) -> Var {
        if let Some(var) = self.var.get(&name) {
            var.clone()
        } else {
            self.offset += 8;
            let var = Var::new(self.offset);
            self.var.insert(name, var.clone());
            var
        }
    }
}
