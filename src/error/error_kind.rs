use std::fmt::Debug;

use crate::{file::position::Position, tokenize::token_kind::TokenKind};

pub enum ErrorKind {
    IO(Box<dyn Debug>),
    Tokenize(TokenizeErrorKind),
    Parse(ParseErrorKind),
    Unexpected,
}

pub enum TokenizeErrorKind {
    UnexpectedChar(Position, char),
}

pub enum ParseErrorKind {
    UnexpectedToken(Position, TokenKind, TokenKind),
}
