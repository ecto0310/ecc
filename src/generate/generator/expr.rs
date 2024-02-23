use std::{fs::File, io::BufWriter, io::Write};

use anyhow::anyhow;

use crate::{
    analyze::expr::{BinaryOpKind, Expr, ExprKind, FuncCallKind},
    generate::register::Register,
};

use super::Generator;

impl Generator {
    pub fn generate_expr(&mut self, f: &mut BufWriter<File>, expr: Expr) -> anyhow::Result<()> {
        match expr.kind {
            ExprKind::Binary { op_kind, lhs, rhs } => {
                self.generate_expr_binary(f, op_kind, *lhs, *rhs)?;
            }
            ExprKind::Assign { op_kind, lhs, rhs } => {
                self.generate_expr_assign(f, op_kind, *lhs, *rhs)?;
            }
            ExprKind::Comma { lhs, rhs } => {
                self.generate_expr(f, *lhs)?;
                self.generate_pop(f, Register::Rax)?;
                self.generate_expr(f, *rhs)?;
            }
            ExprKind::Condition {
                condition,
                then_expr,
                else_expr,
            } => self.generate_expr_condition(f, *condition, *then_expr, *else_expr)?,
            ExprKind::PostfixIncrement { expr } => {
                self.generate_expr_postfix_increment(f, *expr)?
            }
            ExprKind::PostfixDecrement { expr } => {
                self.generate_expr_postfix_decrement(f, *expr)?
            }
            ExprKind::Variable { .. } => {
                self.generate_expr_var(f, expr)?;
            }
            ExprKind::Number { number } => {
                self.generate_expr_number(f, number)?;
            }
            ExprKind::Func { name, args } => self.generate_expr_func(f, name, args)?,
        }
        Ok(())
    }

    fn generate_expr_binary(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: BinaryOpKind,
        lhs: Expr,
        rhs: Expr,
    ) -> anyhow::Result<()> {
        self.generate_expr(f, lhs)?;
        self.generate_expr(f, rhs)?;
        self.generate_pop(f, Register::Rdi)?;
        self.generate_pop(f, Register::Rax)?;
        self.generate_expr_binary_with_reg(f, op_kind, Register::Rax, Register::Rdi)?;
        Ok(())
    }

    fn generate_expr_binary_with_reg(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: BinaryOpKind,
        lhs: Register,
        rhs: Register,
    ) -> anyhow::Result<()> {
        if lhs != Register::Rax {
            writeln!(f, "\tmov {}, {}", Register::Rax.qword(), lhs.qword())?;
        }
        if rhs != Register::Rdi {
            writeln!(f, "\tmov {}, {}", Register::Rdi.qword(), rhs.qword())?;
        }
        match op_kind {
            BinaryOpKind::Add => {
                writeln!(
                    f,
                    "\tadd {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::Sub => {
                writeln!(
                    f,
                    "\tsub {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::Mul => {
                writeln!(
                    f,
                    "\timul {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::Div => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Register::Rdi.qword())?;
            }
            BinaryOpKind::Rem => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Register::Rdi.qword())?;
                writeln!(
                    f,
                    "\tmov {}, {}",
                    Register::Rax.qword(),
                    Register::Rdx.qword()
                )?;
            }
            BinaryOpKind::BitAnd => {
                writeln!(
                    f,
                    "\tand {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::BitOr => {
                writeln!(
                    f,
                    "\tor {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::BitXor => {
                writeln!(
                    f,
                    "\txor {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
            }
            BinaryOpKind::LShift => {
                writeln!(
                    f,
                    "\tmov {}, {}",
                    Register::Rcx.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(
                    f,
                    "\tshl {}, {}",
                    Register::Rax.qword(),
                    Register::Rcx.byte()
                )?;
            }
            BinaryOpKind::RShift => {
                writeln!(
                    f,
                    "\tmov {}, {}",
                    Register::Rcx.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(
                    f,
                    "\tshr {}, {}",
                    Register::Rax.qword(),
                    Register::Rcx.byte()
                )?;
            }
            BinaryOpKind::Lt => {
                writeln!(
                    f,
                    "\tcmp {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(f, "\tsetl {}", Register::Rax.byte())?;
                writeln!(
                    f,
                    "\tmovzb {}, {}",
                    Register::Rax.qword(),
                    Register::Rax.byte()
                )?;
            }
            BinaryOpKind::LtEqual => {
                writeln!(
                    f,
                    "\tcmp {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(f, "\tsetle {}", Register::Rax.byte())?;
                writeln!(
                    f,
                    "\tmovzb {}, {}",
                    Register::Rax.qword(),
                    Register::Rax.byte()
                )?;
            }
            BinaryOpKind::Equal => {
                writeln!(
                    f,
                    "\tcmp {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(f, "\tsete {}", Register::Rax.byte())?;
                writeln!(
                    f,
                    "\tmovzb {}, {}",
                    Register::Rax.qword(),
                    Register::Rax.byte()
                )?;
            }
            BinaryOpKind::NotEqual => {
                writeln!(
                    f,
                    "\tcmp {}, {}",
                    Register::Rax.qword(),
                    Register::Rdi.qword()
                )?;
                writeln!(f, "\tsetne {}", Register::Rax.byte())?;
                writeln!(
                    f,
                    "\tmovzb {}, {}",
                    Register::Rax.qword(),
                    Register::Rax.byte()
                )?;
            }
        }
        self.generate_push_with_reg(f, Register::Rax)?;
        Ok(())
    }

    fn generate_expr_assign(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: BinaryOpKind,
        lhs: Expr,
        rhs: Expr,
    ) -> anyhow::Result<()> {
        if op_kind == BinaryOpKind::Equal {
            self.generate_expr_left_var(f, lhs)?;
            self.generate_expr(f, rhs)?;
            self.generate_pop(f, Register::Rdi)?;
            self.generate_pop(f, Register::Rax)?;
            writeln!(
                f,
                "\tmov {}, [{}]",
                Register::R8.qword(),
                Register::Rax.qword()
            )?;
            self.generate_push_with_reg(f, Register::Rax)?;
            self.generate_expr_binary_with_reg(f, op_kind, Register::R8, Register::Rdi)?;
            self.generate_pop(f, Register::Rdi)?;
            self.generate_pop(f, Register::Rax)?;
            writeln!(
                f,
                "\tmov [{}], {}",
                Register::Rax.qword(),
                Register::Rdi.qword()
            )?;
            self.generate_push_with_reg(f, Register::Rdi)?;
        } else {
            self.generate_expr_left_var(f, lhs)?;
            self.generate_expr(f, rhs)?;
            self.generate_pop(f, Register::Rdi)?;
            self.generate_pop(f, Register::Rax)?;
            writeln!(
                f,
                "\tmov [{}], {}",
                Register::Rax.qword(),
                Register::Rdi.qword()
            )?;
            self.generate_push_with_reg(f, Register::Rdi)?;
        }
        Ok(())
    }

    fn generate_expr_condition(
        &mut self,
        f: &mut BufWriter<File>,
        condition: Expr,
        then_expr: Expr,
        else_expr: Expr,
    ) -> anyhow::Result<()> {
        let label_num = self.label_num();
        self.generate_expr(f, condition)?;
        self.generate_pop(f, Register::Rax)?;
        writeln!(f, "\tcmp {}, 0", Register::Rax.qword())?;
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
        expr: Expr,
    ) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Register::Rdi)?;
        writeln!(
            f,
            "\tmov {}, [{}]",
            Register::Rax.qword(),
            Register::Rdi.qword()
        )?;
        self.generate_push_with_reg(f, Register::Rax)?;
        writeln!(f, "\tadd {}, 1", Register::Rax.qword())?;
        writeln!(
            f,
            "\tmov [{}], {}",
            Register::Rdi.qword(),
            Register::Rax.qword()
        )?;
        Ok(())
    }

    fn generate_expr_postfix_decrement(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Expr,
    ) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Register::Rdi)?;
        writeln!(
            f,
            "\tmov {}, [{}]",
            Register::Rax.qword(),
            Register::Rdi.qword()
        )?;
        self.generate_push_with_reg(f, Register::Rax)?;
        writeln!(f, "\tsub {}, 1", Register::Rax.qword())?;
        writeln!(
            f,
            "\tmov [{}], {}",
            Register::Rdi.qword(),
            Register::Rax.qword()
        )?;
        Ok(())
    }

    fn generate_expr_func(
        &mut self,
        f: &mut BufWriter<File>,
        name: FuncCallKind,
        args: Vec<Expr>,
    ) -> anyhow::Result<()> {
        let arg_len = args.len();
        let stack: usize = if 6 < arg_len { arg_len - 6 } else { 0 };
        let stack_adjust = (self.stack + stack) % 2 == 1;
        if stack_adjust {
            writeln!(f, "\tsub {}, 8", Register::Rsp.qword())?;
            self.stack += 1;
        }
        self.generate_expr_func_args(f, args)?;
        match name {
            FuncCallKind::Label { name } => {
                self.generate_set_func_args(f, arg_len)?;
                writeln!(f, "\tcall {}", name)?;
            }
            FuncCallKind::Expr { expr } => {
                self.generate_expr(f, *expr)?;
                self.generate_pop(f, Register::R10)?;
                self.generate_set_func_args(f, arg_len)?;
                writeln!(f, "\tcall {}", Register::R10.qword())?;
            }
        };
        if stack_adjust {
            writeln!(f, "\tadd {}, {}", Register::Rsp.qword(), (stack + 1) * 8)?;
            self.stack -= stack + 1;
        } else {
            writeln!(f, "\tadd {}, {}", Register::Rsp.qword(), stack * 8)?;
            self.stack -= stack;
        }
        self.generate_push_with_reg(f, Register::Rax)?;
        Ok(())
    }

    fn generate_expr_func_args(
        &mut self,
        f: &mut BufWriter<File>,
        args: Vec<Expr>,
    ) -> anyhow::Result<()> {
        for arg in args.into_iter().rev() {
            self.generate_expr(f, arg)?;
        }
        Ok(())
    }

    fn generate_set_func_args(
        &mut self,
        f: &mut BufWriter<File>,
        length: usize,
    ) -> anyhow::Result<()> {
        let regs = vec![
            Register::Rdi,
            Register::Rsi,
            Register::Rdx,
            Register::Rcx,
            Register::R8,
            Register::R9,
        ];
        for reg in regs.iter().take(std::cmp::min(length, 6)) {
            self.generate_pop(f, reg.clone())?;
        }
        Ok(())
    }

    fn generate_expr_left_var(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Expr,
    ) -> anyhow::Result<()> {
        match expr.kind {
            ExprKind::Variable { var } => {
                writeln!(
                    f,
                    "\tmov {}, {}",
                    Register::Rax.qword(),
                    Register::Rbp.qword()
                )?;
                writeln!(f, "\tsub {}, {}", Register::Rax.qword(), var.offset)?;
            }
            _ => {
                return Err(anyhow!(
                    "{}Must be a changeable left-hand side value",
                    expr.position
                ))
            }
        }
        self.generate_push_with_reg(f, Register::Rax)?;
        Ok(())
    }

    fn generate_expr_var(&mut self, f: &mut BufWriter<File>, expr: Expr) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Register::Rax)?;
        writeln!(
            f,
            "\tmov {}, [{}]",
            Register::Rax.qword(),
            Register::Rax.qword()
        )?;
        self.generate_push_with_reg(f, Register::Rax)?;
        Ok(())
    }

    fn generate_expr_number(
        &mut self,
        f: &mut BufWriter<File>,
        number: usize,
    ) -> anyhow::Result<()> {
        let number = number as i32;
        self.generate_push_with_num(f, number)?;
        Ok(())
    }
}
