mod expr;
mod stmt;

use std::collections::VecDeque;

use crate::tokenize::token_stream::TokenStream;

use super::row_program::RowProgram;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, token_stream: &mut TokenStream) -> anyhow::Result<RowProgram> {
        let mut stmts = VecDeque::new();
        while !token_stream.at_eof()? {
            let stmt = self.parse_stmt(token_stream)?;
            stmts.push_back(stmt);
        }
        Ok(RowProgram::new(stmts))
    }
}
