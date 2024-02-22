use crate::file::position::Position;

use super::{token::Token, token_kind::TokenKind};
use anyhow::anyhow;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn consume(&mut self, kind: TokenKind) -> anyhow::Result<bool> {
        let token = self.peek()?;
        Ok(*token.kind == kind)
    }

    pub fn expect(&mut self, kind: TokenKind) -> anyhow::Result<Token> {
        let token = self.next()?;
        if *token.kind == kind {
            return Ok(token);
        }
        Err(anyhow!(format!(
            "{}Got unexpected token `{:?}`. Expect `{:?}`",
            token.position, token.kind, kind
        )))
    }

    pub fn at_eof(&self) -> anyhow::Result<bool> {
        Ok(*self.peek()?.kind == TokenKind::Eof)
    }

    pub fn next(&mut self) -> anyhow::Result<Token> {
        if let Some(token) = self.tokens.pop_front() {
            return Ok(token);
        }
        Err(anyhow!(format!("Failed to peek tokenstreem")))
    }

    pub fn peek(&self) -> anyhow::Result<Token> {
        if let Some(token) = self.tokens.front() {
            return Ok(token.clone());
        }
        Err(anyhow!(format!("Failed to peek tokenstreem")))
    }

    pub fn get_position(&self) -> anyhow::Result<Position> {
        let token = self.peek()?;
        Ok(token.position)
    }
}
