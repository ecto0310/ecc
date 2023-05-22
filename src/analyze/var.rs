#[derive(Debug, Clone)]
pub struct Var {
    pub offset: usize,
}

impl Var {
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }
}
