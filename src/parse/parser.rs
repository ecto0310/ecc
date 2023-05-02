use std::collections::VecDeque;

use crate::{
    error::error::Error,
    tokenize::{
        token::Token,
        token_kind::{PuncToken, TokenKind},
        token_stream::TokenStream,
    },
};

use super::{
    expression::{AssignOpKind, BinaryOpKind, Expr},
    syntax_tree::SyntaxTree,
};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, token_stream: &mut TokenStream) -> Result<SyntaxTree, Error> {
        let mut exprs = VecDeque::new();
        while !token_stream.at_eof()? {
            if let Some(_) = token_stream.consume(TokenKind::Punc(PuncToken::Semicolon)) {
                continue;
            }
            let expr = self.parse_expression(token_stream)?;
            exprs.push_back(expr);
            token_stream.expect(TokenKind::Punc(PuncToken::Semicolon))?;
        }
        Ok(SyntaxTree::new(exprs))
    }

    fn parse_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut expr = self.parse_assignment_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::Comma))
        {
            expr = Expr::new_comma(
                expr,
                self.parse_assignment_expression(token_stream)?,
                position,
            )
        }
        Ok(expr)
    }

    fn parse_assignment_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut tmp_token_stream = token_stream.clone();
        let lhs = self.parse_unary_expression(&mut tmp_token_stream)?;
        if let Some(Token { kind, position }) = tmp_token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Equal => AssignOpKind::Equal,
                    PuncToken::AsteriskEqual => AssignOpKind::MulEqual,
                    PuncToken::SlashEqual => AssignOpKind::DivEqual,
                    PuncToken::PercentEqual => AssignOpKind::RemEqual,
                    PuncToken::PlusEqual => AssignOpKind::AddEqual,
                    PuncToken::MinusEqual => AssignOpKind::SubEqual,
                    PuncToken::LtLtEqual => AssignOpKind::LShiftEqual,
                    PuncToken::GtGtEqual => AssignOpKind::RShiftEqual,
                    PuncToken::AndEqual => AssignOpKind::BitAndEqual,
                    PuncToken::HatEqual => AssignOpKind::BitXorEqual,
                    PuncToken::VertEqual => AssignOpKind::BitOrEqual,
                    _ => return Ok(self.parse_conditional_expression(token_stream)?),
                },
                _ => return Ok(self.parse_conditional_expression(token_stream)?),
            };
            tmp_token_stream.next();
            let rhs = self.parse_assignment_expression(&mut tmp_token_stream)?;
            *token_stream = tmp_token_stream;
            return Ok(Expr::new_assign(op_kind, lhs, rhs, position));
        }
        return Err(Error::new_unexpected());
    }

    fn parse_conditional_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let condition = self.parse_logical_or_expression(token_stream)?;
        if let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::Question))
        {
            let then_expr = self.parse_expression(token_stream)?;
            token_stream.expect(TokenKind::Punc(PuncToken::Colon))?;
            let else_expr = self.parse_conditional_expression(token_stream)?;
            return Ok(Expr::new_condition(
                condition, then_expr, else_expr, position,
            ));
        }
        Ok(condition)
    }

    fn parse_logical_or_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_logical_and_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::VertVert))
        {
            let rhs = self.parse_logical_and_expression(token_stream)?;
            lhs = Expr::new_binary(BinaryOpKind::LogicOr, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_logical_and_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_inclusive_or_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::AndAnd))
        {
            let rhs = self.parse_inclusive_or_expression(token_stream)?;
            lhs = Expr::new_binary(BinaryOpKind::LogicAnd, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_inclusive_or_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_exclusive_or_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::Vert))
        {
            let rhs = self.parse_exclusive_or_expression(token_stream)?;
            lhs = Expr::new_binary(BinaryOpKind::BitOr, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_exclusive_or_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_and_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::Hat))
        {
            let rhs = self.parse_and_expression(token_stream)?;
            lhs = Expr::new_binary(BinaryOpKind::BitXor, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_and_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut lhs = self.parse_equality_expression(token_stream)?;
        while let Some(Token { position, .. }) =
            token_stream.consume(TokenKind::Punc(PuncToken::And))
        {
            let rhs = self.parse_equality_expression(token_stream)?;
            lhs = Expr::new_binary(BinaryOpKind::BitAnd, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_equality_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut lhs = self.parse_relational_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::EqualEqual => BinaryOpKind::Eq,
                    PuncToken::ExclEqual => BinaryOpKind::Ne,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next();
            let rhs = self.parse_relational_expression(token_stream)?;
            lhs = Expr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_relational_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_shift_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Lt => BinaryOpKind::Lt,
                    PuncToken::Gt => BinaryOpKind::Gt,
                    PuncToken::LtEqual => BinaryOpKind::LtEqual,
                    PuncToken::GtEqual => BinaryOpKind::GtEqual,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next();
            let rhs = self.parse_shift_expression(token_stream)?;
            lhs = Expr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_shift_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut lhs = self.parse_additive_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::LtLt => BinaryOpKind::LShift,
                    PuncToken::GtGt => BinaryOpKind::RShift,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next();
            let rhs = self.parse_additive_expression(token_stream)?;
            lhs = Expr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_additive_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut lhs = self.parse_multiplicative_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Plus => BinaryOpKind::Add,
                    PuncToken::Minus => BinaryOpKind::Sub,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next();
            let rhs = self.parse_multiplicative_expression(token_stream)?;
            lhs = Expr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_multiplicative_expression(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> Result<Expr, Error> {
        let mut lhs = self.parse_cast_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Asterisk => BinaryOpKind::Mul,
                    PuncToken::Slash => BinaryOpKind::Div,
                    PuncToken::Percent => BinaryOpKind::Rem,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next();
            let rhs = self.parse_cast_expression(token_stream)?;
            lhs = Expr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_cast_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        self.parse_unary_expression(token_stream)
    }

    fn parse_unary_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let (kind, position) = match token_stream.peek() {
            Some(Token { kind, position }) => (kind, position),
            None => return Err(Error::new_unexpected()),
        };
        Ok(match *kind {
            TokenKind::Punc(punc) => match punc {
                PuncToken::PlusPlus => {
                    token_stream.next();
                    Expr::new_unary_increment(
                        self.parse_postfix_expression(token_stream)?,
                        position,
                    )
                }
                PuncToken::MinusMinus => {
                    token_stream.next();
                    Expr::new_unary_decrement(
                        self.parse_postfix_expression(token_stream)?,
                        position,
                    )
                }
                _ => self.parse_postfix_expression(token_stream)?,
            },
            _ => self.parse_postfix_expression(token_stream)?,
        })
    }

    fn parse_postfix_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let mut expr = self.parse_primary_expression(token_stream)?;
        while let Some(Token { kind, position }) = token_stream.peek() {
            match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::PlusPlus => {
                        token_stream.next();
                        expr = Expr::new_postfix_increment(expr, position);
                    }
                    PuncToken::MinusMinus => {
                        token_stream.next();
                        expr = Expr::new_postfix_decrement(expr, position);
                    }
                    _ => break,
                },
                _ => break,
            };
        }
        Ok(expr)
    }

    fn parse_primary_expression(&mut self, token_stream: &mut TokenStream) -> Result<Expr, Error> {
        let (kind, position) = match token_stream.next() {
            Some(Token { kind, position }) => (kind, position),
            None => return Err(Error::new_unexpected()),
        };
        let expr = match *kind {
            TokenKind::Number(number) => Expr::new_number(number, position),
            TokenKind::Ident(ident) => Expr::new_ident(ident, position),
            TokenKind::Punc(PuncToken::OpenRound) => {
                let expr = self.parse_expression(token_stream)?;
                token_stream.expect(TokenKind::Punc(PuncToken::CloseRound))?;
                expr
            }
            _ => return Err(Error::new_unexpected()),
        };
        return Ok(expr);
    }
}
