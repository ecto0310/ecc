use std::{fmt, rc::Rc};

use super::file_info::FileInfo;

#[derive(Clone, Debug)]
pub struct Position {
    file_info: Rc<FileInfo>,
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(file_info: Rc<FileInfo>) -> Self {
        Self {
            file_info,
            line: 1,
            column: 1,
        }
    }

    pub fn add_column(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }

    pub fn get_position(&self) -> (&str, usize, &str, usize) {
        let code = self
            .file_info
            .get_code()
            .split('\n')
            .nth(self.line - 1)
            .unwrap();
        (self.file_info.get_name(), self.line, code, self.column)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (file_name, line, code, indent) = self.get_position();
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
