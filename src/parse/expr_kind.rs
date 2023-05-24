use crate::{analyze::gen_expr_kind::GenBinaryOpKind, error::Error};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary {
        op_kind: BinaryOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        op_kind: AssignOpKind,
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
    UnaryIncrement {
        expr: Box<Expr>,
    },
    UnaryDecrement {
        expr: Box<Expr>,
    },
    PostfixIncrement {
        expr: Box<Expr>,
    },
    PostfixDecrement {
        expr: Box<Expr>,
    },
    Identifier {
        name: String,
    },
    Number {
        number: usize,
    },
    Func {
        name: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
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
    /// logical AND operator ('&&')
    LogicAnd,
    /// logical OR operator ('||')
    LogicOr,
    /// left-shift operator ('<<')
    LShift,
    /// right-shift operator ('>>')
    RShift,
    /// less-than operator ('<')
    Lt,
    /// greater-than operator ('>')
    Gt,
    /// less-than-or-equal-to operator ('<=')
    LtEqual,
    /// greater-than-or-equal-to operator ('>=')
    GtEqual,
    /// equality operator ('==')
    Eq,
    /// inequality operator ('!=')
    Ne,
}

impl BinaryOpKind {
    pub fn convert_to_gen(&self) -> Result<GenBinaryOpKind, Error> {
        match self {
            BinaryOpKind::Add => Ok(GenBinaryOpKind::Add),
            BinaryOpKind::Sub => Ok(GenBinaryOpKind::Sub),
            BinaryOpKind::Mul => Ok(GenBinaryOpKind::Mul),
            BinaryOpKind::Div => Ok(GenBinaryOpKind::Div),
            BinaryOpKind::Rem => Ok(GenBinaryOpKind::Rem),
            BinaryOpKind::BitAnd => Ok(GenBinaryOpKind::BitAnd),
            BinaryOpKind::BitOr => Ok(GenBinaryOpKind::BitOr),
            BinaryOpKind::BitXor => Ok(GenBinaryOpKind::BitXor),
            BinaryOpKind::LogicAnd => Err(Error::new_unexpected()),
            BinaryOpKind::LogicOr => Err(Error::new_unexpected()),
            BinaryOpKind::LShift => Ok(GenBinaryOpKind::LShift),
            BinaryOpKind::RShift => Ok(GenBinaryOpKind::RShift),
            BinaryOpKind::Lt => Ok(GenBinaryOpKind::Lt),
            BinaryOpKind::Gt => Ok(GenBinaryOpKind::Lt),
            BinaryOpKind::LtEqual => Ok(GenBinaryOpKind::LtEqual),
            BinaryOpKind::GtEqual => Ok(GenBinaryOpKind::LtEqual),
            BinaryOpKind::Eq => Ok(GenBinaryOpKind::Eq),
            BinaryOpKind::Ne => Ok(GenBinaryOpKind::Ne),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOpKind {
    /// simple assignment operator ('=')
    Equal,
    /// multiplication assignment operator ('*=')
    MulEqual,
    /// division assignment operator ('/=')
    DivEqual,
    /// remainder assignment operator ('%=')
    RemEqual,
    /// addition assignment operator ('+=')
    AddEqual,
    /// subtraction assignment operator ('-=')
    SubEqual,
    /// left-shift assignment operator ('<<=')
    LShiftEqual,
    /// right-shift assignment operator ('>>=')
    RShiftEqual,
    /// bitwise AND assignment operator ('&=')
    BitAndEqual,
    /// bitwise exclusive OR assignment operator ('^=')
    BitXorEqual,
    /// bitwise inclusive OR assignment operator ('|=')
    BitOrEqual,
}

impl AssignOpKind {
    pub fn convert_to_binary(&self) -> Result<GenBinaryOpKind, Error> {
        match self {
            AssignOpKind::Equal => Err(Error::new_unexpected()),
            AssignOpKind::MulEqual => Ok(GenBinaryOpKind::Mul),
            AssignOpKind::DivEqual => Ok(GenBinaryOpKind::Div),
            AssignOpKind::RemEqual => Ok(GenBinaryOpKind::Rem),
            AssignOpKind::AddEqual => Ok(GenBinaryOpKind::Add),
            AssignOpKind::SubEqual => Ok(GenBinaryOpKind::Sub),
            AssignOpKind::LShiftEqual => Ok(GenBinaryOpKind::LShift),
            AssignOpKind::RShiftEqual => Ok(GenBinaryOpKind::RShift),
            AssignOpKind::BitAndEqual => Ok(GenBinaryOpKind::BitAnd),
            AssignOpKind::BitXorEqual => Ok(GenBinaryOpKind::BitXor),
            AssignOpKind::BitOrEqual => Ok(GenBinaryOpKind::BitOr),
        }
    }
}
