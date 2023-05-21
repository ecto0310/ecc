pub mod error_kind;

use std::{fmt::Debug, io};

use crate::{file::position::Position, tokenize::token_kind::TokenKind};

use error_kind::{ErrorKind, ParseErrorKind, TokenizeErrorKind};

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new_io(error: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::IO(Box::new(error)),
        }
    }
    pub fn new_unexpected_char(position: Position, char: char) -> Self {
        Self {
            kind: ErrorKind::Tokenize(TokenizeErrorKind::UnexpectedChar(position, char)),
        }
    }
    pub fn new_unexpected_token(
        position: Position,
        expected: TokenKind,
        actual: TokenKind,
    ) -> Self {
        Self {
            kind: ErrorKind::Parse(ParseErrorKind::UnexpectedToken(position, expected, actual)),
        }
    }
    pub fn new_unexpected() -> Self {
        Self {
            kind: ErrorKind::Unexpected,
        }
    }

    fn error_at(&self, position: &Position, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file_name, line, code, indent) = position.get_position();
        writeln!(
            f,
            "{}:{}\n{}\n{}^",
            file_name,
            line,
            code,
            " ".repeat(indent - 1)
        )
    }
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        Error::new_unexpected()
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IO(err) => Debug::fmt(err, f)?,
            ErrorKind::Tokenize(TokenizeErrorKind::UnexpectedChar(position, char)) => {
                self.error_at(position, f)?;
                writeln!(f, "Got Unexpected char in tokenizing: `{}`", char)?;
            }
            ErrorKind::Parse(ParseErrorKind::UnexpectedToken(position, expected, actual)) => {
                self.error_at(position, f)?;
                writeln!(
                    f,
                    "Got Unexpected token in parse: `{:?}` (expected: `{:?}`)",
                    actual, expected
                )?;
            }
            ErrorKind::Unexpected => {
                writeln!(f, "Got Unexpected error")?;
            }
        }
        Ok(())
    }
}
