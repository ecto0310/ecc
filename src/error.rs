use std::{fmt::Debug, io};

use crate::{file::position::Position, tokenize::token_kind::TokenKind};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{error:?}")]
    IO { error: Box<dyn Debug> },
    #[error("{position}unexpected char")]
    TokenizeUnexpectedChar { position: Position, char: char },
    #[error("{position}unexpected token")]
    ParseUnexpectedToken {
        position: Position,
        expected: TokenKind,
        actual: TokenKind,
    },
    #[error("Unexpected Error")]
    Unexpected,
}

impl Error {
    pub fn new_io(error: std::io::Error) -> Self {
        Self::IO {
            error: Box::new(error),
        }
    }
    pub fn new_unexpected_char(position: Position, char: char) -> Self {
        Self::TokenizeUnexpectedChar { position, char }
    }
    pub fn new_unexpected_token(
        position: Position,
        expected: TokenKind,
        actual: TokenKind,
    ) -> Self {
        Self::ParseUnexpectedToken {
            position,
            expected,
            actual,
        }
    }
    pub fn new_unexpected() -> Self {
        Self::Unexpected
    }
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Self::new_unexpected()
    }
}
