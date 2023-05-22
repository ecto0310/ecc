use crate::file::position::Position;

use super::{
    gen_expr_kind::{GenBinaryOpKind, GenExprKind},
    variable::Variable,
};

#[derive(Debug, Clone)]
pub struct GenExpr {
    pub kind: GenExprKind,
    pub position: Position,
}

impl GenExpr {
    pub fn new_binary(
        op_kind: GenBinaryOpKind,
        lhs: GenExpr,
        rhs: GenExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: GenExprKind::Binary {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_assign(lhs: GenExpr, rhs: GenExpr, position: Position) -> Self {
        Self {
            kind: GenExprKind::Assign {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_assign_op(
        op_kind: GenBinaryOpKind,
        lhs: GenExpr,
        rhs: GenExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: GenExprKind::AssignOP {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_postfix_increment(expr: GenExpr, position: Position) -> Self {
        Self {
            kind: GenExprKind::PostfixIncrement {
                expr: Box::new(expr),
            },
            position,
        }
    }

    pub fn new_postfix_decrement(expr: GenExpr, position: Position) -> Self {
        Self {
            kind: GenExprKind::PostfixDecrement {
                expr: Box::new(expr),
            },
            position,
        }
    }

    pub fn new_comma(lhs: GenExpr, rhs: GenExpr, position: Position) -> Self {
        Self {
            kind: GenExprKind::Comma {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_condition(
        condition: GenExpr,
        then_expr: GenExpr,
        else_expr: GenExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: GenExprKind::Condition {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            },
            position,
        }
    }

    pub fn new_variable(variable: Variable, position: Position) -> Self {
        Self {
            kind: GenExprKind::Variable { variable },
            position,
        }
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        Self {
            kind: GenExprKind::Number { number },
            position,
        }
    }
}
