use crate::file::position::Position;

#[derive(Debug)]
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
            kind: ExprKind::UnaryIncrement(Box::new(expr)),
            position,
        };
    }

    pub fn new_unary_decrement(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::UnaryDecrement(Box::new(expr)),
            position,
        };
    }

    pub fn new_postfix_increment(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::PostfixIncrement(Box::new(expr)),
            position,
        };
    }

    pub fn new_postfix_decrement(expr: Expr, position: Position) -> Self {
        return Expr {
            kind: ExprKind::PostfixDecrement(Box::new(expr)),
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

    pub fn new_ident(ident: String, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Variable(ident),
            position,
        };
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        return Expr {
            kind: ExprKind::Number(number),
            position,
        };
    }
}

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
    Variable(String),
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
