use crate::{
    parse::row_stmt::RowStmt,
    tokenize::{
        token_kind::{PuncToken, TokenKind},
        token_stream::TokenStream,
    },
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let stmt = if token_stream.consume(TokenKind::Return)? {
            self.parse_return_stmt(token_stream)?
        } else if token_stream.consume(TokenKind::If)? {
            self.parse_if_stmt(token_stream)?
        } else if token_stream.consume(TokenKind::For)? {
            self.parse_for_stmt(token_stream)?
        } else if token_stream.consume(TokenKind::While)? {
            self.parse_while_stmt(token_stream)?
        } else if token_stream.consume(TokenKind::Punc(PuncToken::OpenCurly))? {
            self.parse_cpd_stmt(token_stream)?
        } else {
            self.parse_expr_stmt(token_stream)?
        };
        Ok(stmt)
    }

    fn parse_return_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.expect(TokenKind::Return)?;
        let stmt = if token_stream.consume(TokenKind::Punc(PuncToken::Semicolon))? {
            token_stream.next()?;
            RowStmt::new_return(None, token.position)
        } else {
            let expr = self.parse_expr(token_stream)?;
            token_stream.expect(TokenKind::Punc(PuncToken::Semicolon))?;
            RowStmt::new_return(Some(expr), token.position)
        };
        Ok(stmt)
    }

    fn parse_if_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.expect(TokenKind::If)?;
        token_stream.expect(TokenKind::Punc(PuncToken::OpenRound))?;
        let condition = self.parse_expr(token_stream)?;
        token_stream.expect(TokenKind::Punc(PuncToken::CloseRound))?;
        let then_stmt = self.parse_stmt(token_stream)?;
        if token_stream.consume(TokenKind::Else)? {
            token_stream.next()?;
            let else_stmt = self.parse_stmt(token_stream)?;
            return Ok(RowStmt::new_if(
                condition,
                then_stmt,
                Some(else_stmt),
                token.position,
            ));
        }
        Ok(RowStmt::new_if(condition, then_stmt, None, token.position))
    }

    fn parse_for_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.expect(TokenKind::For)?;
        token_stream.expect(TokenKind::Punc(PuncToken::OpenRound))?;
        let init = if token_stream.consume(TokenKind::Punc(PuncToken::Semicolon))? {
            None
        } else {
            let condition = self.parse_expr(token_stream)?;
            Some(condition)
        };
        token_stream.expect(TokenKind::Punc(PuncToken::Semicolon))?;
        let condition = if token_stream.consume(TokenKind::Punc(PuncToken::Semicolon))? {
            None
        } else {
            let condition = self.parse_expr(token_stream)?;
            Some(condition)
        };
        token_stream.expect(TokenKind::Punc(PuncToken::Semicolon))?;
        let delta = if token_stream.consume(TokenKind::Punc(PuncToken::CloseRound))? {
            None
        } else {
            let condition = self.parse_expr(token_stream)?;
            Some(condition)
        };
        token_stream.expect(TokenKind::Punc(PuncToken::CloseRound))?;
        let run_stmt = self.parse_stmt(token_stream)?;
        Ok(RowStmt::new_for(
            init,
            condition,
            delta,
            run_stmt,
            token.position,
        ))
    }

    fn parse_while_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.expect(TokenKind::While)?;
        token_stream.expect(TokenKind::Punc(PuncToken::OpenRound))?;
        let condition = self.parse_expr(token_stream)?;
        token_stream.expect(TokenKind::Punc(PuncToken::CloseRound))?;
        let run_stmt = self.parse_stmt(token_stream)?;
        Ok(RowStmt::new_while(condition, run_stmt, token.position))
    }

    fn parse_cpd_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.expect(TokenKind::Punc(PuncToken::OpenCurly))?;
        let mut stmts = vec![];
        while !token_stream.consume(TokenKind::Punc(PuncToken::CloseCurly))? {
            stmts.push(self.parse_stmt(token_stream)?);
        }
        token_stream.next()?;
        Ok(RowStmt::new_cpd(stmts, token.position))
    }

    fn parse_expr_stmt(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowStmt> {
        let token = token_stream.peek()?;
        if *token.kind == TokenKind::Punc(PuncToken::Semicolon) {
            return Ok(RowStmt::new_expr(None, token.position));
        }
        let expr = self.parse_expr(token_stream)?;
        token_stream.expect(TokenKind::Punc(PuncToken::Semicolon))?;
        Ok(RowStmt::new_expr(Some(expr), token.position))
    }
}
