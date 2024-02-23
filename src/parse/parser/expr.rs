use anyhow::anyhow;

use crate::{
    parse::row_expr::{RowAssignOpKind, RowBinaryOpKind, RowExpr},
    tokenize::{
        token::Token,
        token_kind::{PuncToken, TokenKind},
        token_stream::TokenStream,
    },
};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut expr = self.parse_assignment_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::Comma))? {
            token_stream.next()?;
            let position = token_stream.get_position()?;
            expr = RowExpr::new_comma(expr, self.parse_assignment_expr(token_stream)?, position)
        }
        Ok(expr)
    }

    fn parse_assignment_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut tmp_token_stream = token_stream.clone();
        let lhs = self.parse_unary_expr(&mut tmp_token_stream)?;
        let token = tmp_token_stream.peek()?;
        let op_kind = match *token.kind {
            TokenKind::Punc(punc) => match punc {
                PuncToken::Equal => RowAssignOpKind::Equal,
                PuncToken::AsteriskEqual => RowAssignOpKind::MulEqual,
                PuncToken::SlashEqual => RowAssignOpKind::DivEqual,
                PuncToken::PercentEqual => RowAssignOpKind::RemEqual,
                PuncToken::PlusEqual => RowAssignOpKind::AddEqual,
                PuncToken::MinusEqual => RowAssignOpKind::SubEqual,
                PuncToken::LtLtEqual => RowAssignOpKind::LShiftEqual,
                PuncToken::GtGtEqual => RowAssignOpKind::RShiftEqual,
                PuncToken::AndEqual => RowAssignOpKind::BitAndEqual,
                PuncToken::HatEqual => RowAssignOpKind::BitXorEqual,
                PuncToken::VertEqual => RowAssignOpKind::BitOrEqual,
                _ => return self.parse_conditional_expr(token_stream),
            },
            _ => return self.parse_conditional_expr(token_stream),
        };
        tmp_token_stream.next()?;
        let rhs = self.parse_assignment_expr(&mut tmp_token_stream)?;
        *token_stream = tmp_token_stream;
        Ok(RowExpr::new_assign(op_kind, lhs, rhs, token.position))
    }

    fn parse_conditional_expr(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> anyhow::Result<RowExpr> {
        let condition = self.parse_logical_or_expr(token_stream)?;
        if token_stream.consume(TokenKind::Punc(PuncToken::Question))? {
            let token = token_stream.next()?;
            let then_expr = self.parse_expr(token_stream)?;
            token_stream.expect(TokenKind::Punc(PuncToken::Colon))?;
            let else_expr = self.parse_conditional_expr(token_stream)?;
            return Ok(RowExpr::new_condition(
                condition,
                then_expr,
                else_expr,
                token.position,
            ));
        }
        Ok(condition)
    }

    fn parse_logical_or_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_logical_and_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::VertVert))? {
            let token = token_stream.next()?;
            let rhs = self.parse_logical_and_expr(token_stream)?;
            lhs = RowExpr::new_binary(RowBinaryOpKind::LogicOr, lhs, rhs, token.position);
        }
        Ok(lhs)
    }

    fn parse_logical_and_expr(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_inclusive_or_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::AndAnd))? {
            let token = token_stream.next()?;
            let rhs = self.parse_inclusive_or_expr(token_stream)?;
            lhs = RowExpr::new_binary(RowBinaryOpKind::LogicAnd, lhs, rhs, token.position);
        }
        Ok(lhs)
    }

    fn parse_inclusive_or_expr(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_exclusive_or_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::Vert))? {
            let token = token_stream.next()?;
            let rhs = self.parse_exclusive_or_expr(token_stream)?;
            lhs = RowExpr::new_binary(RowBinaryOpKind::BitOr, lhs, rhs, token.position);
        }
        Ok(lhs)
    }

    fn parse_exclusive_or_expr(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_and_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::Hat))? {
            let token = token_stream.next()?;
            let rhs = self.parse_and_expr(token_stream)?;
            lhs = RowExpr::new_binary(RowBinaryOpKind::BitXor, lhs, rhs, token.position);
        }
        Ok(lhs)
    }

    fn parse_and_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_equality_expr(token_stream)?;
        while token_stream.consume(TokenKind::Punc(PuncToken::And))? {
            let token = token_stream.next()?;
            let rhs = self.parse_equality_expr(token_stream)?;
            lhs = RowExpr::new_binary(RowBinaryOpKind::BitAnd, lhs, rhs, token.position);
        }
        Ok(lhs)
    }

    fn parse_equality_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_relational_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::EqualEqual => RowBinaryOpKind::Equal,
                    PuncToken::ExclEqual => RowBinaryOpKind::NotEqual,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next()?;
            let rhs = self.parse_relational_expr(token_stream)?;
            lhs = RowExpr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_relational_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_shift_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Lt => RowBinaryOpKind::Lt,
                    PuncToken::Gt => RowBinaryOpKind::Gt,
                    PuncToken::LtEqual => RowBinaryOpKind::LtEqual,
                    PuncToken::GtEqual => RowBinaryOpKind::GtEqual,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next()?;
            let rhs = self.parse_shift_expr(token_stream)?;
            lhs = RowExpr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_shift_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_additive_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::LtLt => RowBinaryOpKind::LShift,
                    PuncToken::GtGt => RowBinaryOpKind::RShift,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next()?;
            let rhs = self.parse_additive_expr(token_stream)?;
            lhs = RowExpr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_additive_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_multiplicative_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Plus => RowBinaryOpKind::Add,
                    PuncToken::Minus => RowBinaryOpKind::Sub,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next()?;
            let rhs = self.parse_multiplicative_expr(token_stream)?;
            lhs = RowExpr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_multiplicative_expr(
        &mut self,
        token_stream: &mut TokenStream,
    ) -> anyhow::Result<RowExpr> {
        let mut lhs = self.parse_cast_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            let op_kind = match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::Asterisk => RowBinaryOpKind::Mul,
                    PuncToken::Slash => RowBinaryOpKind::Div,
                    PuncToken::Percent => RowBinaryOpKind::Rem,
                    _ => break,
                },
                _ => break,
            };
            token_stream.next()?;
            let rhs = self.parse_cast_expr(token_stream)?;
            lhs = RowExpr::new_binary(op_kind, lhs, rhs, position);
        }
        Ok(lhs)
    }

    fn parse_cast_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        self.parse_unary_expr(token_stream)
    }

    fn parse_unary_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let Token { kind, position } = token_stream.peek()?;
        Ok(match *kind {
            TokenKind::Punc(punc) => match punc {
                PuncToken::PlusPlus => {
                    token_stream.next()?;
                    RowExpr::new_unary_increment(self.parse_postfix_expr(token_stream)?, position)
                }
                PuncToken::MinusMinus => {
                    token_stream.next()?;
                    RowExpr::new_unary_decrement(self.parse_postfix_expr(token_stream)?, position)
                }
                _ => self.parse_postfix_expr(token_stream)?,
            },
            _ => self.parse_postfix_expr(token_stream)?,
        })
    }

    fn parse_postfix_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let mut expr = self.parse_primary_expr(token_stream)?;
        loop {
            let Token { kind, position } = token_stream.peek()?;
            match *kind {
                TokenKind::Punc(punc) => match punc {
                    PuncToken::PlusPlus => {
                        token_stream.next()?;
                        expr = RowExpr::new_postfix_increment(expr, position);
                    }
                    PuncToken::MinusMinus => {
                        token_stream.next()?;
                        expr = RowExpr::new_postfix_decrement(expr, position);
                    }
                    PuncToken::OpenRound => {
                        token_stream.next()?;
                        let mut args = Vec::new();
                        if token_stream.consume(TokenKind::Punc(PuncToken::CloseRound))? {
                            token_stream.next()?;
                            return Ok(RowExpr::new_func(expr, args, position));
                        }
                        args.push(self.parse_assignment_expr(token_stream)?);
                        while !token_stream.consume(TokenKind::Punc(PuncToken::CloseRound))? {
                            token_stream.expect(TokenKind::Punc(PuncToken::Comma))?;
                            args.push(self.parse_assignment_expr(token_stream)?);
                        }
                        token_stream.next()?;
                        return Ok(RowExpr::new_func(expr, args, position));
                    }
                    _ => break,
                },
                _ => break,
            };
        }
        Ok(expr)
    }

    fn parse_primary_expr(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowExpr> {
        let token = token_stream.next()?;
        let expr = match *token.kind {
            TokenKind::Number(number) => RowExpr::new_number(number, token.position),
            TokenKind::Ident(name) => RowExpr::new_ident(name, token.position),
            TokenKind::Punc(PuncToken::OpenRound) => {
                let expr = self.parse_expr(token_stream)?;
                token_stream.expect(TokenKind::Punc(PuncToken::CloseRound))?;
                expr
            }
            _ => {
                return Err(anyhow!(format!(
                    "{}Got unexpected token `{:?}`. Expect primary",
                    token.position, token.kind
                )))
            }
        };
        Ok(expr)
    }
}
