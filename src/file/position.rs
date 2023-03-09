use crate::FileInfo;
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
            line: 0,
            column: 0,
        }
    }

    pub fn add_column(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.column = 0;
        self.line += 1;
    }
}
