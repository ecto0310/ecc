use crate::error::Error;

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

    pub fn consume(&mut self, kind: TokenKind) -> Option<Token> {
        if let Some(token) = self.peek() {
            if *token.kind == kind {
                return self.next();
            }
        }
        None
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        if let Some(token) = self.next() {
            if *token.kind == kind {
                return Ok(token);
            }
            return Err(Error::new_unexpected_token(
                token.position,
                kind,
                *token.kind,
            ));
        }
        Err(Error::new_unexpected())
    }

    pub fn at_eof(&self) -> Result<bool, Error> {
        if let Some(Token { kind, .. }) = self.peek() {
            return Ok(*kind == TokenKind::Eof);
        }
        Err(Error::new_unexpected())
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.tokens.len() <= 1 {
            return self.peek();
        }
        self.tokens.pop_front()
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.front().cloned()
    }
}
