use super::{gen_expr::GenExpr, variable::Variable};

#[derive(Debug)]
pub enum GenExprKind {
    Binary {
        op_kind: GenBinaryOpKind,
        lhs: Box<GenExpr>,
        rhs: Box<GenExpr>,
    },
    Assign {
        lhs: Box<GenExpr>,
        rhs: Box<GenExpr>,
    },
    AssignOP {
        op_kind: GenBinaryOpKind,
        lhs: Box<GenExpr>,
        rhs: Box<GenExpr>,
    },
    Comma {
        lhs: Box<GenExpr>,
        rhs: Box<GenExpr>,
    },
    Condition {
        condition: Box<GenExpr>,
        then_expr: Box<GenExpr>,
        else_expr: Box<GenExpr>,
    },
    PostfixIncrement {
        expr: Box<GenExpr>,
    },
    PostfixDecrement {
        expr: Box<GenExpr>,
    },
    Variable {
        variable: Variable,
    },
    Number {
        number: usize,
    },
}

#[derive(Debug)]
pub enum GenBinaryOpKind {
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
