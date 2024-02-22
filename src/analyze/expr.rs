use anyhow::anyhow;

use crate::{
    file::position::Position,
    parse::row_expr::{RowAssignOpKind, RowBinaryOpKind},
};

use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub position: Position,
}

impl Expr {
    pub fn new_binary(op_kind: BinaryOpKind, lhs: Expr, rhs: Expr, position: Position) -> Self {
        Self {
            kind: ExprKind::Binary {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_assign(op_kind: BinaryOpKind, lhs: Expr, rhs: Expr, position: Position) -> Self {
        Self {
            kind: ExprKind::Assign {
                op_kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_postfix_increment(expr: Expr, position: Position) -> Self {
        Self {
            kind: ExprKind::PostfixIncrement {
                expr: Box::new(expr),
            },
            position,
        }
    }

    pub fn new_postfix_decrement(expr: Expr, position: Position) -> Self {
        Self {
            kind: ExprKind::PostfixDecrement {
                expr: Box::new(expr),
            },
            position,
        }
    }

    pub fn new_comma(lhs: Expr, rhs: Expr, position: Position) -> Self {
        Self {
            kind: ExprKind::Comma {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            position,
        }
    }

    pub fn new_condition(
        condition: Expr,
        then_expr: Expr,
        else_expr: Expr,
        position: Position,
    ) -> Self {
        Self {
            kind: ExprKind::Condition {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            },
            position,
        }
    }

    pub fn new_var(var: Variable, position: Position) -> Self {
        Self {
            kind: ExprKind::Variable { var },
            position,
        }
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        Self {
            kind: ExprKind::Number { number },
            position,
        }
    }

    pub fn new_func_expr(expr: Expr, args: Vec<Expr>, position: Position) -> Self {
        Self {
            kind: ExprKind::Func {
                name: FuncCallKind::Expr {
                    expr: Box::new(expr),
                },
                args,
            },
            position,
        }
    }

    pub fn new_func_label(name: String, args: Vec<Expr>, position: Position) -> Self {
        Self {
            kind: ExprKind::Func {
                name: FuncCallKind::Label { name },
                args,
            },
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary {
        op_kind: BinaryOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        op_kind: BinaryOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Comma {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Condition {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    PostfixIncrement {
        expr: Box<Expr>,
    },
    PostfixDecrement {
        expr: Box<Expr>,
    },
    Variable {
        var: Variable,
    },
    Number {
        number: usize,
    },
    Func {
        name: FuncCallKind,
        args: Vec<Expr>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOpKind {
    /// addition operator ('+')
    Add,
    /// subtraction operator ('-')
    Sub,
    /// multiplication operator ('*')
    Mul,
    /// division operator ('/')
    Div,
    /// remainder operator ('%')
    Rem,
    /// bitwise AND operator ('&')
    BitAnd,
    /// bitwise inclusive OR operator ('|')
    BitOr,
    /// bitwise exclusive OR operator ('^')
    BitXor,
    /// left-shift operator ('<<')
    LShift,
    /// right-shift operator ('>>')
    RShift,
    /// less-than operator ('<')
    Lt,
    /// less-than-or-equal-to operator ('<=')
    LtEqual,
    /// equality operator ('==')
    Eq,
    /// inequality operator ('!=')
    Ne,
}

impl BinaryOpKind {
    pub fn from_row_binary_op_kind(row: RowBinaryOpKind) -> anyhow::Result<Self> {
        match row {
            RowBinaryOpKind::Add => Ok(Self::Add),
            RowBinaryOpKind::Sub => Ok(Self::Sub),
            RowBinaryOpKind::Mul => Ok(Self::Mul),
            RowBinaryOpKind::Div => Ok(Self::Div),
            RowBinaryOpKind::Rem => Ok(Self::Rem),
            RowBinaryOpKind::BitAnd => Ok(Self::BitAnd),
            RowBinaryOpKind::BitOr => Ok(Self::BitOr),
            RowBinaryOpKind::BitXor => Ok(Self::BitXor),
            RowBinaryOpKind::LogicAnd => Err(anyhow!("Unexpected LogicAnd operater convert")),
            RowBinaryOpKind::LogicOr => Err(anyhow!("Unexpected LogicOr operater convert")),
            RowBinaryOpKind::LShift => Ok(Self::LShift),
            RowBinaryOpKind::RShift => Ok(Self::RShift),
            RowBinaryOpKind::Lt => Ok(Self::Lt),
            RowBinaryOpKind::Gt => Ok(Self::Lt),
            RowBinaryOpKind::LtEqual => Ok(Self::LtEqual),
            RowBinaryOpKind::GtEqual => Ok(Self::LtEqual),
            RowBinaryOpKind::Eq => Ok(Self::Eq),
            RowBinaryOpKind::Ne => Ok(Self::Ne),
        }
    }

    pub fn from_row_assign_op_kind(row: RowAssignOpKind) -> anyhow::Result<Self> {
        match row {
            RowAssignOpKind::Equal => Ok(Self::Eq),
            RowAssignOpKind::MulEqual => Ok(Self::Mul),
            RowAssignOpKind::DivEqual => Ok(Self::Div),
            RowAssignOpKind::RemEqual => Ok(Self::Rem),
            RowAssignOpKind::AddEqual => Ok(Self::Add),
            RowAssignOpKind::SubEqual => Ok(Self::Sub),
            RowAssignOpKind::LShiftEqual => Ok(Self::LShift),
            RowAssignOpKind::RShiftEqual => Ok(Self::RShift),
            RowAssignOpKind::BitAndEqual => Ok(Self::BitAnd),
            RowAssignOpKind::BitXorEqual => Ok(Self::BitXor),
            RowAssignOpKind::BitOrEqual => Ok(Self::BitOr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FuncCallKind {
    Label { name: String },
    Expr { expr: Box<Expr> },
}
