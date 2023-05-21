use crate::file::position::Position;

use super::expr_kind::{AssignOpKind, BinaryOpKind, ExprKind};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub position: Position,
}

impl Expr {
    pub fn new_binary(op_kind: BinaryOpKind, lhs: Expr, rhs: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Binary {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        };
    }

    pub fn new_assign(op_kind: AssignOpKind, lhs: Expr, rhs: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Assign {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        };
    }

    pub fn new_unary_increment(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::UnaryIncrement {
                expr: Box::new(expr),
            },
            position,
        };
    }

    pub fn new_unary_decrement(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::UnaryDecrement {
                expr: Box::new(expr),
            },
            position,
        };
    }

    pub fn new_postfix_increment(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::PostfixIncrement {
                expr: Box::new(expr),
            },
            position,
        };
    }

    pub fn new_postfix_decrement(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::PostfixDecrement {
                expr: Box::new(expr),
            },
            position,
        };
    }

    pub fn new_comma(lhs: Expr, rhs: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Comma {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        };
    }

    pub fn new_condition(
        condition: Expr,
        then_expr: Expr,
        else_expr: Expr,
        position: Position,
    ) -> Self {
        return Expr {
            kind: ExprKind::Condition {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            },
            position,
        };
    }

    pub fn new_ident(name: String, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Identifier { name },
            position,
        };
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Number { number },
            position,
        };
    }
}
