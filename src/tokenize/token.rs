use crate::file::position::Position;

use super::token_kind::{PuncToken, TokenKind};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Box<TokenKind>,
    pub position: Position,
}

impl Token {
    pub fn new_punc_token(punc_token: PuncToken, position: Position) -> Self {
        Self {
            kind: Box::new(TokenKind::Punc(punc_token)),
            position,
        }
    }

    pub fn new_number(number: usize, position: Position) -> Self {
        Self {
            kind: Box::new(TokenKind::Number(number)),
            position,
        }
    }

    pub fn new(kind: TokenKind, position: Position) -> Self {
        Self {
            kind: Box::new(kind),
            position,
        }
    }

    pub fn new_eof(position: Position) -> Self {
        Self {
            kind: Box::new(TokenKind::Eof),
            position,
        }
    }
}
