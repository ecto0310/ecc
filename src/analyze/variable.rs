#[derive(Debug, Clone)]
pub struct Variable {
    pub offset: usize,
}

impl Variable {
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }
}
