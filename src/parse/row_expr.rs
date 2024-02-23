use crate::file::position::Position;

#[derive(Debug, Clone)]
pub struct RowExpr {
    pub kind: RowExprKind,
    pub position: Position,
}

impl RowExpr {
    pub fn new_binary(
        row_binary_op_kind: RowBinaryOpKind,
        row_lhs_expr: RowExpr,
        row_rhs_expr: RowExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: RowExprKind::Binary {
                row_binary_op_kind,
                row_lhs_expr: Box::new(row_lhs_expr),
                row_rhs_expr: Box::new(row_rhs_expr),
            },
            position,
        }
    }

    pub fn new_assign(
        row_assign_op_kind: RowAssignOpKind,
        row_lhs_expr: RowExpr,
        row_rhs_expr: RowExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: RowExprKind::Assign {
                row_assign_op_kind,
                row_lhs_expr: Box::new(row_lhs_expr),
                row_rhs_expr: Box::new(row_rhs_expr),
            },
            position,
        }
    }

    pub fn new_unary_increment(row_expr: RowExpr, position: Position) -> Self {
        Self {
            kind: RowExprKind::UnaryIncrement {
                row_expr: Box::new(row_expr),
            },
            position,
        }
    }

    pub fn new_unary_decrement(row_expr: RowExpr, position: Position) -> Self {
        Self {
            kind: RowExprKind::UnaryDecrement {
                row_expr: Box::new(row_expr),
            },
            position,
        }
    }

    pub fn new_postfix_increment(row_expr: RowExpr, position: Position) -> Self {
        Self {
            kind: RowExprKind::PostfixIncrement {
                row_expr: Box::new(row_expr),
            },
            position,
        }
    }

    pub fn new_postfix_decrement(row_expr: RowExpr, position: Position) -> Self {
        Self {
            kind: RowExprKind::PostfixDecrement {
                row_expr: Box::new(row_expr),
            },
            position,
        }
    }

    pub fn new_comma(row_lhs_expr: RowExpr, row_rhs_expr: RowExpr, position: Position) -> Self {
        Self {
            kind: RowExprKind::Comma {
                row_lhs_expr: Box::new(row_lhs_expr),
                row_rhs_expr: Box::new(row_rhs_expr),
            },
            position,
        }
    }

    pub fn new_condition(
        row_condition_expr: RowExpr,
        row_then_expr: RowExpr,
        row_else_expr: RowExpr,
        position: Position,
    ) -> Self {
        Self {
            kind: RowExprKind::Condition {
                row_condition_expr: Box::new(row_condition_expr),
                row_then_expr: Box::new(row_then_expr),
                row_else_expr: Box::new(row_else_expr),
            },
            position,
        }
    }

    pub fn new_ident(ident: String, position: Position) -> Self {
        Self {
            kind: RowExprKind::Identifier { ident },
            position,
        }
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        Self {
            kind: RowExprKind::Number { number },
            position,
        }
    }

    pub fn new_func(
        row_name_expr: RowExpr,
        row_args_expr: Vec<RowExpr>,
        position: Position,
    ) -> Self {
        Self {
            kind: RowExprKind::Func {
                row_name_expr: Box::new(row_name_expr),
                row_args_expr,
            },
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RowExprKind {
    Binary {
        row_binary_op_kind: RowBinaryOpKind,
        row_lhs_expr: Box<RowExpr>,
        row_rhs_expr: Box<RowExpr>,
    },
    Assign {
        row_assign_op_kind: RowAssignOpKind,
        row_lhs_expr: Box<RowExpr>,
        row_rhs_expr: Box<RowExpr>,
    },
    Comma {
        row_lhs_expr: Box<RowExpr>,
        row_rhs_expr: Box<RowExpr>,
    },
    Condition {
        row_condition_expr: Box<RowExpr>,
        row_then_expr: Box<RowExpr>,
        row_else_expr: Box<RowExpr>,
    },
    UnaryIncrement {
        row_expr: Box<RowExpr>,
    },
    UnaryDecrement {
        row_expr: Box<RowExpr>,
    },
    PostfixIncrement {
        row_expr: Box<RowExpr>,
    },
    PostfixDecrement {
        row_expr: Box<RowExpr>,
    },
    Identifier {
        ident: String,
    },
    Number {
        number: usize,
    },
    Func {
        row_name_expr: Box<RowExpr>,
        row_args_expr: Vec<RowExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum RowBinaryOpKind {
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
    Equal,
    /// inequality operator ('!=')
    NotEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RowAssignOpKind {
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
