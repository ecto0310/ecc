use std::{fmt::Debug, io};

use crate::{file::position::Position, tokenize::token::Token};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{:?}", error)]
    IO { error: Box<dyn Debug> },
    #[error("{}Got unexpected char `{}`", position, char)]
    TokenizeUnexpectedChar { position: Position, char: char },
    #[error("{}Got unexpected token `{:?}`. Expect `{}`.", got.position, got.kind, expect)]
    ParseUnexpectedToken { got: Token, expect: String },
    #[error("Got unexpected error.")]
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
    pub fn new_unexpected_token(got: Token, expect: String) -> Self {
        Self::ParseUnexpectedToken { got, expect }
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
