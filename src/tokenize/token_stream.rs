use crate::{error::Error, file::position::Position};

use super::{token::Token, token_kind::TokenKind};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn consume(&mut self, kind: TokenKind) -> Result<bool, Error> {
        let token = self.peek()?;
        Ok(*token.kind == kind)
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        let token = self.next()?;
        if *token.kind == kind {
            return Ok(token);
        }
        Err(Error::new_unexpected_token(token, format!("{:?}", kind)))
    }

    pub fn at_eof(&self) -> Result<bool, Error> {
        Ok(*self.peek()?.kind == TokenKind::Eof)
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        if let Some(token) = self.tokens.pop_front() {
            return Ok(token);
        }
        Err(Error::new_unexpected())
    }

    pub fn peek(&self) -> Result<Token, Error> {
        if let Some(token) = self.tokens.front() {
            return Ok(token.clone());
        }
        Err(Error::new_unexpected())
    }

    pub fn get_position(&self) -> Result<Position, Error> {
        let token = self.peek()?;
        Ok(token.position)
    }
}
