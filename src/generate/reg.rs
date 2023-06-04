#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum Reg {
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

impl Reg {
    pub fn qword(&self) -> &str {
        match self {
            Reg::Rax => "rax",
            Reg::Rbx => "rbx",
            Reg::Rcx => "rcx",
            Reg::Rdx => "rdx",
            Reg::Rsi => "rsi",
            Reg::Rdi => "rdi",
            Reg::Rbp => "rbp",
            Reg::Rsp => "rsp",
            Reg::R8 => "r8",
            Reg::R9 => "r9",
            Reg::R10 => "r10",
            Reg::R11 => "r11",
            Reg::R12 => "r12",
            Reg::R13 => "r13",
            Reg::R14 => "r14",
            Reg::R15 => "r15",
        }
    }

    #[allow(dead_code)]
    pub fn dword(&self) -> &str {
        match self {
            Reg::Rax => "eax",
            Reg::Rbx => "ebx",
            Reg::Rcx => "ecx",
            Reg::Rdx => "edx",
            Reg::Rsi => "esi",
            Reg::Rdi => "edi",
            Reg::Rbp => "ebp",
            Reg::Rsp => "esp",
            Reg::R8 => "r8d",
            Reg::R9 => "r9d",
            Reg::R10 => "r10d",
            Reg::R11 => "r11d",
            Reg::R12 => "r12d",
            Reg::R13 => "r13d",
            Reg::R14 => "r14d",
            Reg::R15 => "r15d",
        }
    }

    #[allow(dead_code)]
    pub fn word(&self) -> &str {
        match self {
            Reg::Rax => "ax",
            Reg::Rbx => "bx",
            Reg::Rcx => "cx",
            Reg::Rdx => "dx",
            Reg::Rsi => "si",
            Reg::Rdi => "di",
            Reg::Rbp => "bp",
            Reg::Rsp => "sp",
            Reg::R8 => "r8w",
            Reg::R9 => "r9w",
            Reg::R10 => "r10w",
            Reg::R11 => "r11w",
            Reg::R12 => "r12w",
            Reg::R13 => "r13w",
            Reg::R14 => "r14w",
            Reg::R15 => "r15w",
        }
    }

    pub fn byte(&self) -> &str {
        match self {
            Reg::Rax => "al",
            Reg::Rbx => "bl",
            Reg::Rcx => "cl",
            Reg::Rdx => "dl",
            Reg::Rsi => "sil",
            Reg::Rdi => "dil",
            Reg::Rbp => "bpl",
            Reg::Rsp => "spl",
            Reg::R8 => "r8b",
            Reg::R9 => "r9b",
            Reg::R10 => "r10b",
            Reg::R11 => "r11b",
            Reg::R12 => "r12b",
            Reg::R13 => "r13b",
            Reg::R14 => "r14b",
            Reg::R15 => "r15b",
        }
    }
}
