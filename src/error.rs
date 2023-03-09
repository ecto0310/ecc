use std::fmt::{Debug, Display};

pub struct CompileError {
    kind: CompileErrorKind,
}

impl CompileError {
    pub fn io(error: std::io::Error) -> Self {
        Self {
            kind: CompileErrorKind::IOError(Box::new(error)),
        }
    }
}

pub enum CompileErrorKind {
    IOError(Box<dyn Debug>),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CompileErrorKind::IOError(err) => Debug::fmt(err, f),
        }
    }
}
