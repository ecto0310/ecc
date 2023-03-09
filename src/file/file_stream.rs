use crate::FileInfo;
use std::collections::VecDeque;
use std::rc::Rc;

use super::position::Position;

pub struct FileStream {
    chars: VecDeque<(Position, char)>,
}

impl FileStream {
    pub fn new(file_info: Rc<FileInfo>) -> Self {
        let code: Vec<char> = file_info.get_code().chars().collect();
        let mut position = Position::new(file_info);
        let mut chars = VecDeque::new();

        for c in code {
            chars.push_back((position.clone(), c));
            if c == '\n' {
                position.new_line()
            } else {
                position.add_column()
            }
        }
        Self { chars }
    }

    pub fn next(&mut self) -> Option<(Position, char)> {
        self.chars.pop_front()
    }

    pub fn peek(&self) -> Option<&(Position, char)> {
        self.chars.front()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        if self.chars.len() < prefix.len() {
            return false;
        }
        self.chars
            .iter()
            .map(|(_, c)| c)
            .take(prefix.len())
            .zip(prefix.chars())
            .all(|(a, b)| *a == b)
    }

    pub fn starts_with_number(&self) -> bool {
        matches!(self.peek(), Some((_, '0'..='9')))
    }

    pub fn starts_with_alphabet(&self) -> bool {
        matches!(self.peek(), Some((_, 'a'..='z' | 'A'..='Z')))
    }

    pub fn advance(&mut self, times: usize) {
        for _ in 0..times {
            self.next();
        }
    }
}
