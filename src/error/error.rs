use std::fmt::Debug;

use crate::file::position::Position;

use super::error_kind::{ErrorKind, TokenizeErrorKind};

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

    fn error_at(&self, position: &Position, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file_name, line, code, indent) = position.get_position();
        writeln!(
            f,
            "{}:{}\n{}\n{}^",
            file_name,
            line,
            code,
            " ".repeat(indent)
        )
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
        }
        Ok(())
    }
}
