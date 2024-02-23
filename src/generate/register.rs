#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register {
    pub fn qword(&self) -> &str {
        match self {
            Self::Rax => "rax",
            Self::Rbx => "rbx",
            Self::Rcx => "rcx",
            Self::Rdx => "rdx",
            Self::Rsi => "rsi",
            Self::Rdi => "rdi",
            Self::Rbp => "rbp",
            Self::Rsp => "rsp",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        }
    }

    #[allow(dead_code)]
    pub fn dword(&self) -> &str {
        match self {
            Self::Rax => "eax",
            Self::Rbx => "ebx",
            Self::Rcx => "ecx",
            Self::Rdx => "edx",
            Self::Rsi => "esi",
            Self::Rdi => "edi",
            Self::Rbp => "ebp",
            Self::Rsp => "esp",
            Self::R8 => "r8d",
            Self::R9 => "r9d",
            Self::R10 => "r10d",
            Self::R11 => "r11d",
            Self::R12 => "r12d",
            Self::R13 => "r13d",
            Self::R14 => "r14d",
            Self::R15 => "r15d",
        }
    }

    #[allow(dead_code)]
    pub fn word(&self) -> &str {
        match self {
            Self::Rax => "ax",
            Self::Rbx => "bx",
            Self::Rcx => "cx",
            Self::Rdx => "dx",
            Self::Rsi => "si",
            Self::Rdi => "di",
            Self::Rbp => "bp",
            Self::Rsp => "sp",
            Self::R8 => "r8w",
            Self::R9 => "r9w",
            Self::R10 => "r10w",
            Self::R11 => "r11w",
            Self::R12 => "r12w",
            Self::R13 => "r13w",
            Self::R14 => "r14w",
            Self::R15 => "r15w",
        }
    }

    pub fn byte(&self) -> &str {
        match self {
            Self::Rax => "al",
            Self::Rbx => "bl",
            Self::Rcx => "cl",
            Self::Rdx => "dl",
            Self::Rsi => "sil",
            Self::Rdi => "dil",
            Self::Rbp => "bpl",
            Self::Rsp => "spl",
            Self::R8 => "r8b",
            Self::R9 => "r9b",
            Self::R10 => "r10b",
            Self::R11 => "r11b",
            Self::R12 => "r12b",
            Self::R13 => "r13b",
            Self::R14 => "r14b",
            Self::R15 => "r15b",
        }
    }
}
