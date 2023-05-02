use super::expr::Expr;

#[derive(Debug)]
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
    UnaryIncrement(Box<Expr>),
    UnaryDecrement(Box<Expr>),
    PostfixIncrement(Box<Expr>),
    PostfixDecrement(Box<Expr>),
    Identifier(String),
    Number(usize),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
