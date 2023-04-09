use std::fmt::Debug;

use crate::file::position::Position;

pub enum ErrorKind {
    IO(Box<dyn Debug>),
    Tokenize(TokenizeErrorKind),
}

pub enum TokenizeErrorKind {
    UnexpectedChar(Position, char),
}
