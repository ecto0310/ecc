use super::file_info::FileInfo;
use std::rc::Rc;

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
