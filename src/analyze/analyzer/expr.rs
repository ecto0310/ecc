use crate::{
    analyze::expr::{BinaryOpKind, Expr},
    file::position::Position,
    parse::row_expr::{RowAssignOpKind, RowBinaryOpKind, RowExpr, RowExprKind},
};

use super::Analyzer;

impl Analyzer {
    pub fn analyze_expr(&mut self, row_expr: RowExpr) -> anyhow::Result<Expr> {
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
}
