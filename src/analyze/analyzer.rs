use std::collections::{BTreeMap, VecDeque};

use crate::{
    error::Error,
    file::position::Position,
    parse::{
        expr::Expr,
        expr_kind::{AssignOpKind, BinaryOpKind, ExprKind},
        syntax_tree::SyntaxTree,
    },
};

use super::{
    gen_expr::GenExpr, gen_expr_kind::GenBinaryOpKind, gen_tree::GenTree, variable::Variable,
};

pub struct Analyzer {
    variable: BTreeMap<String, Variable>,
    offset: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            variable: BTreeMap::new(),
            offset: 0,
        }
    }

    pub fn analyze(&mut self, syntax_tree: SyntaxTree) -> Result<GenTree, Error> {
        let mut gen_exprs = VecDeque::new();
        // for expr in syntax_tree.exprs.into_iter() {
        //     gen_exprs.push_back(self.analyze_expression(expr)?)
        // }
        Ok(GenTree::new(gen_exprs, self.offset))
    }

    fn analyze_expression(&mut self, expr: Expr) -> Result<GenExpr, Error> {
        let position = expr.position;
        Ok(match expr.kind {
            ExprKind::Binary { op_kind, lhs, rhs } => {
                self.analyze_expression_binary(op_kind, *lhs, *rhs, position)?
            }
            ExprKind::Assign { op_kind, lhs, rhs } => {
                self.analyze_expression_assign(op_kind, *lhs, *rhs, position)?
            }
            ExprKind::Comma { lhs, rhs } => GenExpr::new_comma(
                self.analyze_expression(*lhs)?,
                self.analyze_expression(*rhs)?,
                position,
            ),
            ExprKind::Condition {
                condition,
                then_expr,
                else_expr,
            } => GenExpr::new_condition(
                self.analyze_expression(*condition)?,
                self.analyze_expression(*then_expr)?,
                self.analyze_expression(*else_expr)?,
                position,
            ),
            ExprKind::UnaryIncrement { expr } => GenExpr::new_assign_op(
                GenBinaryOpKind::Add,
                self.analyze_expression(*expr)?,
                GenExpr::new_number(1, position.clone()),
                position,
            ),
            ExprKind::UnaryDecrement { expr } => GenExpr::new_assign_op(
                GenBinaryOpKind::Sub,
                self.analyze_expression(*expr)?,
                GenExpr::new_number(1, position.clone()),
                position,
            ),
            ExprKind::PostfixIncrement { expr } => {
                GenExpr::new_postfix_increment(self.analyze_expression(*expr)?, position)
            }
            ExprKind::PostfixDecrement { expr } => {
                GenExpr::new_postfix_decrement(self.analyze_expression(*expr)?, position)
            }
            ExprKind::Identifier { name } => {
                let variable = self.get_variable(name);
                GenExpr::new_variable(variable, position)
            }
            ExprKind::Number { number } => GenExpr::new_number(number, position),
        })
    }

    fn analyze_expression_binary(
        &mut self,
        op_kind: BinaryOpKind,
        lhs: Expr,
        rhs: Expr,
        position: Position,
    ) -> Result<GenExpr, Error> {
        Ok(match op_kind {
            BinaryOpKind::LogicAnd => GenExpr::new_condition(
                self.analyze_expression(lhs)?,
                self.analyze_expression(rhs)?,
                GenExpr::new_number(0, position.clone()),
                position,
            ),
            BinaryOpKind::LogicOr => GenExpr::new_condition(
                self.analyze_expression(lhs)?,
                GenExpr::new_number(1, position.clone()),
                self.analyze_expression(rhs)?,
                position,
            ),
            BinaryOpKind::Gt => GenExpr::new_binary(
                GenBinaryOpKind::Lt,
                self.analyze_expression(rhs)?,
                self.analyze_expression(lhs)?,
                position,
            ),
            BinaryOpKind::GtEqual => GenExpr::new_binary(
                GenBinaryOpKind::LtEqual,
                self.analyze_expression(rhs)?,
                self.analyze_expression(lhs)?,
                position,
            ),
            op_kind => GenExpr::new_binary(
                op_kind.convert_to_gen()?,
                self.analyze_expression(lhs)?,
                self.analyze_expression(rhs)?,
                position,
            ),
        })
    }

    fn analyze_expression_assign(
        &mut self,
        op_kind: AssignOpKind,
        lhs: Expr,
        rhs: Expr,
        position: Position,
    ) -> Result<GenExpr, Error> {
        let lhs = self.analyze_expression(lhs)?;
        let rhs = self.analyze_expression(rhs)?;
        if op_kind == AssignOpKind::Equal {
            return Ok(GenExpr::new_assign(lhs, rhs, position));
        }
        let op_kind = op_kind.convert_to_binary()?;
        Ok(GenExpr::new_assign_op(op_kind, lhs, rhs, position))
    }

    fn get_variable(&mut self, name: String) -> Variable {
        if let Some(variable) = self.variable.get(&name) {
            variable.clone()
        } else {
            self.offset += 8;
            let variable = Variable::new(self.offset);
            self.variable.insert(name, variable.clone());
            variable
        }
    }
}
