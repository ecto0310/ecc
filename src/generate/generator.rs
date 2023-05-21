use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::analyze::gen_expr::GenExpr;
use crate::analyze::gen_expr_kind::{GenBinaryOpKind, GenExprKind};
use crate::{analyze::gen_tree::GenTree, error::Error};

use super::reg::Reg;

pub struct Generator {
    label: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self { label: 0 }
    }

    pub fn generate(&mut self, f: &mut BufWriter<File>, gen_tree: GenTree) -> Result<(), Error> {
        writeln!(f, ".intel_syntax noprefix")?;
        writeln!(f, ".globl main")?;
        writeln!(f, "main:")?;
        writeln!(f, "\tpush {}", Reg::Rbp.qword())?;
        writeln!(f, "\tmov {}, {}", Reg::Rbp.qword(), Reg::Rsp.qword())?;
        writeln!(f, "\tsub {}, {}", Reg::Rsp.qword(), gen_tree.offset)?;

        for gen_expr in gen_tree.gen_exprs.into_iter() {
            self.generate_expr(f, gen_expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }

        writeln!(f, "\tmov {}, {}", Reg::Rsp.qword(), Reg::Rbp.qword())?;
        self.generate_pop(f, Reg::Rbp)?;
        writeln!(f, "\tret")?;
        Ok(())
    }

    fn generate_expr(&mut self, f: &mut BufWriter<File>, gen_expr: GenExpr) -> Result<(), Error> {
        match gen_expr.kind {
            GenExprKind::Binary { op_kind, lhs, rhs } => {
                self.generate_expr_binary(f, op_kind, *lhs, *rhs)?;
            }
            GenExprKind::Assign { lhs, rhs } => {
                self.generate_expr_assign(f, *lhs, *rhs)?;
            }
            GenExprKind::AssignOP { op_kind, lhs, rhs } => {
                self.generate_expr_assign_op(f, op_kind, *lhs, *rhs)?;
            }
            GenExprKind::Comma { lhs, rhs } => {
                self.generate_expr(f, *lhs)?;
                self.generate_pop(f, Reg::Rax)?;
                self.generate_expr(f, *rhs)?;
            }
            GenExprKind::Condition {
                condition,
                then_expr,
                else_expr,
            } => self.generate_expr_condition(f, *condition, *then_expr, *else_expr)?,
            GenExprKind::PostfixIncrement { expr } => {
                self.generate_expr_postfix_increment(f, *expr)?
            }
            GenExprKind::PostfixDecrement { expr } => {
                self.generate_expr_postfix_decrement(f, *expr)?
            }
            GenExprKind::Variable { .. } => {
                self.generate_expr_variable(f, gen_expr)?;
            }
            GenExprKind::Number { number } => {
                self.generate_expr_number(f, number)?;
            }
        }
        Ok(())
    }

    fn generate_expr_binary(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: GenBinaryOpKind,
        lhs: GenExpr,
        rhs: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr(f, lhs)?;
        self.generate_expr(f, rhs)?;
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        self.generate_expr_binary_with_reg(f, op_kind, Reg::Rax, Reg::Rdi)?;
        Ok(())
    }

    fn generate_expr_binary_with_reg(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: GenBinaryOpKind,
        lhs: Reg,
        rhs: Reg,
    ) -> Result<(), Error> {
        if lhs != Reg::Rax {
            writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), lhs.qword())?;
        }
        if rhs != Reg::Rdi {
            writeln!(f, "\tmov {}, {}", Reg::Rdi.qword(), rhs.qword())?;
        }
        match op_kind {
            GenBinaryOpKind::Add => {
                writeln!(f, "\tadd {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::Sub => {
                writeln!(f, "\tsub {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::Mul => {
                writeln!(f, "\timul {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::Div => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::Rem => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Reg::Rdi.qword())?;
                writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), Reg::Rdx.qword())?;
            }
            GenBinaryOpKind::BitAnd => {
                writeln!(f, "\tand {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::BitOr => {
                writeln!(f, "\tor {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::BitXor => {
                writeln!(f, "\txor {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            GenBinaryOpKind::LShift => {
                writeln!(f, "\tmov {}, {}", Reg::Rcx.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tshl {}, {}", Reg::Rax.qword(), Reg::Rcx.byte())?;
            }
            GenBinaryOpKind::RShift => {
                writeln!(f, "\tmov {}, {}", Reg::Rcx.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tshr {}, {}", Reg::Rax.qword(), Reg::Rcx.byte())?;
            }
            GenBinaryOpKind::Lt => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetl {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            GenBinaryOpKind::LtEqual => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetle {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            GenBinaryOpKind::Eq => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsete {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            GenBinaryOpKind::Ne => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetne {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
        }
        self.generate_push(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_assign(
        &mut self,
        f: &mut BufWriter<File>,
        lhs: GenExpr,
        rhs: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_variable(f, lhs)?;
        self.generate_expr(f, rhs)?;
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov [{}], {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push(f, Reg::Rdi)?;
        Ok(())
    }

    fn generate_expr_assign_op(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: GenBinaryOpKind,
        lhs: GenExpr,
        rhs: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_variable(f, lhs)?;
        self.generate_expr(f, rhs)?;
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov {}, [{}]", Reg::R8.qword(), Reg::Rax.qword())?;
        self.generate_push(f, Reg::Rax)?;
        self.generate_expr_binary_with_reg(f, op_kind, Reg::R8, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov [{}], {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push(f, Reg::Rdi)?;
        Ok(())
    }

    fn generate_expr_condition(
        &mut self,
        f: &mut BufWriter<File>,
        condition: GenExpr,
        then_expr: GenExpr,
        else_expr: GenExpr,
    ) -> Result<(), Error> {
        let label_num = self.label_num();
        self.generate_expr(f, condition)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tcmp {}, 0", Reg::Rax.qword())?;
        writeln!(f, "\tje .Lelse{}", label_num)?;
        self.generate_expr(f, then_expr)?;
        writeln!(f, "\tjmp .Lend{}", label_num)?;
        writeln!(f, ".Lelse{}:", label_num)?;
        self.generate_expr(f, else_expr)?;
        writeln!(f, ".Lend{}:", label_num)?;
        Ok(())
    }

    fn generate_expr_postfix_increment(
        &mut self,
        f: &mut BufWriter<File>,
        gen_expr: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_variable(f, gen_expr)?;
        self.generate_pop(f, Reg::Rdi)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push(f, Reg::Rax)?;
        writeln!(f, "\tadd {}, 1", Reg::Rax.qword())?;
        writeln!(f, "\tmov [{}], {}", Reg::Rdi.qword(), Reg::Rax.qword())?;
        Ok(())
    }

    fn generate_expr_postfix_decrement(
        &mut self,
        f: &mut BufWriter<File>,
        gen_expr: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_variable(f, gen_expr)?;
        self.generate_pop(f, Reg::Rdi)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push(f, Reg::Rax)?;
        writeln!(f, "\tsub {}, 1", Reg::Rax.qword())?;
        writeln!(f, "\tmov [{}], {}", Reg::Rdi.qword(), Reg::Rax.qword())?;
        Ok(())
    }

    fn generate_expr_left_variable(
        &self,
        f: &mut BufWriter<File>,
        gen_expr: GenExpr,
    ) -> Result<(), Error> {
        match gen_expr.kind {
            GenExprKind::Variable { variable } => {
                writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), Reg::Rbp.qword())?;
                writeln!(f, "\tsub {}, {}", Reg::Rax.qword(), variable.offset)?;
            }
            _ => return Err(Error::new_unexpected()),
        }
        self.generate_push(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_variable(
        &self,
        f: &mut BufWriter<File>,
        gen_expr: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_variable(f, gen_expr)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rax.qword())?;
        self.generate_push(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_number(&self, f: &mut BufWriter<File>, number: usize) -> Result<(), Error> {
        writeln!(f, "\tpush {}", number)?;
        Ok(())
    }

    fn generate_push(&self, f: &mut BufWriter<File>, reg: Reg) -> Result<(), Error> {
        writeln!(f, "\tpush {}", reg.qword())?;
        Ok(())
    }

    fn generate_pop(&self, f: &mut BufWriter<File>, reg: Reg) -> Result<(), Error> {
        writeln!(f, "\tpop {}", reg.qword())?;
        Ok(())
    }

    fn label_num(&mut self) -> usize {
        let lebel = self.label;
        self.label += 1;
        lebel
    }
}
