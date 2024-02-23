mod expr;
mod stmt;

use std::{fs::File, io::BufWriter, io::Write};

use crate::analyze::program::Program;

use super::register::Register;

pub struct Generator {
    label: usize,
    stack: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self { label: 0, stack: 0 }
    }

    pub fn generate(&mut self, f: &mut BufWriter<File>, program: Program) -> anyhow::Result<()> {
        writeln!(f, ".intel_syntax noprefix")?;
        writeln!(f, ".globl main")?;
        writeln!(f, "main:")?;
        self.generate_push_with_reg(f, Register::Rbp)?;
        writeln!(
            f,
            "\tmov {}, {}",
            Register::Rbp.qword(),
            Register::Rsp.qword()
        )?;
        let offset = if program.offset % 16 == 0 {
            program.offset
        } else {
            program.offset + program.offset % 16
        };
        writeln!(f, "\tsub {}, {}", Register::Rsp.qword(), offset)?;

        for stmt in program.stmts.into_iter() {
            self.generate_stmt(f, stmt)?;
        }

        writeln!(f, ".Lmain_ret:")?;
        writeln!(
            f,
            "\tmov {}, {}",
            Register::Rsp.qword(),
            Register::Rbp.qword()
        )?;
        self.generate_pop(f, Register::Rbp)?;
        writeln!(f, "\tret")?;
        Ok(())
    }

    fn generate_push_with_reg(
        &mut self,
        f: &mut BufWriter<File>,
        reg: Register,
    ) -> anyhow::Result<()> {
        writeln!(f, "\tpush {}", reg.qword())?;
        self.stack += 1;
        Ok(())
    }

    fn generate_push_with_num(&mut self, f: &mut BufWriter<File>, num: i32) -> anyhow::Result<()> {
        writeln!(f, "\tpush {}", num)?;
        self.stack += 1;
        Ok(())
    }

    fn generate_pop(&mut self, f: &mut BufWriter<File>, reg: Register) -> anyhow::Result<()> {
        writeln!(f, "\tpop {}", reg.qword())?;
        self.stack -= 1;
        Ok(())
    }

    fn label_num(&mut self) -> usize {
        let lebel = self.label;
        self.label += 1;
        lebel
    }
}
