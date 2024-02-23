use std::collections::VecDeque;

use anyhow::anyhow;

use crate::file::file_stream::FileStream;

use super::{
    token::Token,
    token_kind::{PuncToken, TokenKind},
};

pub struct Tokenizer {
    file_stream: FileStream,
}

impl Tokenizer {
    pub fn new(file_stream: FileStream) -> Self {
        Self { file_stream }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<VecDeque<Token>> {
        let mut tokens = VecDeque::new();

        while !self.file_stream.is_empty() {
            if self.file_stream.starts_with_white_space() {
                self.file_stream.advance(1);
                continue;
            }

            if let Some(token) = self.tokenize_symbol() {
                tokens.push_back(token);
                continue;
            }

            if let Some(token) = self.tokenize_number() {
                tokens.push_back(token);
                continue;
            }

            if let Some(token) = self.tokenize_other() {
                tokens.push_back(token);
                continue;
            }

            let (position, char) = self.file_stream.peek().unwrap();
            return Err(anyhow!(format!(
                "{}Got unexpected char `{}`",
                position, char
            )));
        }

        tokens.push_back(Token::new_eof(self.file_stream.advance(1).unwrap().0));
        Ok(tokens)
    }

    fn tokenize_symbol(&mut self) -> Option<Token> {
        let symbols = vec![
            ("...", PuncToken::DotDotDot),
            ("<<=", PuncToken::LtLtEqual),
            (">>=", PuncToken::GtGtEqual),
            ("<:", PuncToken::OpenSquare),
            (":>", PuncToken::CloseSquare),
            ("<%", PuncToken::OpenCurly),
            ("%>", PuncToken::CloseCurly),
            ("->", PuncToken::MinusGt),
            ("++", PuncToken::PlusPlus),
            ("--", PuncToken::MinusMinus),
            ("<<", PuncToken::LtLt),
            (">>", PuncToken::GtGt),
            ("<=", PuncToken::LtEqual),
            (">=", PuncToken::GtEqual),
            ("==", PuncToken::EqualEqual),
            ("!=", PuncToken::ExclEqual),
            ("&&", PuncToken::AndAnd),
            ("||", PuncToken::VertVert),
            ("*=", PuncToken::AsteriskEqual),
            ("/=", PuncToken::SlashEqual),
            ("%=", PuncToken::PercentEqual),
            ("+=", PuncToken::PlusEqual),
            ("-=", PuncToken::MinusEqual),
            ("&=", PuncToken::AndEqual),
            ("^=", PuncToken::HatEqual),
            ("|=", PuncToken::VertEqual),
            ("[", PuncToken::OpenSquare),
            ("]", PuncToken::CloseSquare),
            ("(", PuncToken::OpenRound),
            (")", PuncToken::CloseRound),
            ("{", PuncToken::OpenCurly),
            ("}", PuncToken::CloseCurly),
            (".", PuncToken::Dot),
            ("&", PuncToken::And),
            ("*", PuncToken::Asterisk),
            ("+", PuncToken::Plus),
            ("-", PuncToken::Minus),
            ("~", PuncToken::Tilde),
            ("!", PuncToken::Excl),
            ("/", PuncToken::Slash),
            ("%", PuncToken::Percent),
            ("<", PuncToken::Lt),
            (">", PuncToken::Gt),
            ("^", PuncToken::Hat),
            ("|", PuncToken::Vert),
            ("?", PuncToken::Question),
            (":", PuncToken::Colon),
            (";", PuncToken::Semicolon),
            ("=", PuncToken::Equal),
            (",", PuncToken::Comma),
        ];

        let match_symbol = symbols
            .clone()
            .into_iter()
            .filter(|(literal, _)| self.file_stream.starts_with(literal))
            .take(1)
            .next();
        if let Some((literal, kind)) = match_symbol {
            let position = self.file_stream.advance(literal.len()).unwrap();
            Some(Token::new_punc_token(kind, position.0))
        } else {
            None
        }
    }

    fn tokenize_number(&mut self) -> Option<Token> {
        if self.file_stream.starts_with_number() {
            let (position, char) = self.file_stream.advance(1).unwrap();
            let mut number = String::from(char);
            while self.file_stream.starts_with_number() {
                let (_, char) = self.file_stream.advance(1).unwrap();
                number.push(char);
            }
            let number = number.parse::<usize>().unwrap();
            Some(Token::new_number(number, position))
        } else {
            None
        }
    }

    fn tokenize_other(&mut self) -> Option<Token> {
        if self.file_stream.starts_with_alphabet() || self.file_stream.starts_with("_") {
            let (position, char) = self.file_stream.advance(1).unwrap();
            let mut ident = String::from(char);
            while self.file_stream.starts_with_alphabet()
                || self.file_stream.starts_with("_")
                || self.file_stream.starts_with_number()
            {
                let (_, char) = self.file_stream.advance(1).unwrap();
                ident.push(char);
            }
            let kind = match ident.as_str() {
                "return" => TokenKind::Return,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "for" => TokenKind::For,
                "while" => TokenKind::While,
                _ => TokenKind::Ident(ident),
            };
            Some(Token::new(kind, position))
        } else {
            None
        }
    }
}
