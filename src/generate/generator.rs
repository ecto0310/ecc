use std::io::Write;
use std::{fs::File, io::BufWriter};

use anyhow::anyhow;

use crate::analyze::expr::{BinaryOpKind, Expr, ExprKind, FuncCallKind};
use crate::analyze::program::Program;
use crate::analyze::stmt::{Stmt, StmtKind};

use super::reg::Reg;

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
        self.generate_push_with_reg(f, Reg::Rbp)?;
        writeln!(f, "\tmov {}, {}", Reg::Rbp.qword(), Reg::Rsp.qword())?;
        let offset = if program.offset % 16 == 0 {
            program.offset
        } else {
            program.offset + program.offset % 16
        };
        writeln!(f, "\tsub {}, {}", Reg::Rsp.qword(), offset)?;

        for stmt in program.stmts.into_iter() {
            self.generate_stmt(f, stmt)?;
        }

        writeln!(f, ".Lmain_ret:")?;
        writeln!(f, "\tmov {}, {}", Reg::Rsp.qword(), Reg::Rbp.qword())?;
        self.generate_pop(f, Reg::Rbp)?;
        writeln!(f, "\tret")?;
        Ok(())
    }

    fn generate_stmt(&mut self, f: &mut BufWriter<File>, stmt: Stmt) -> anyhow::Result<()> {
        match stmt.kind {
            StmtKind::Expr { expr } => {
                self.generate_stmt_expr(f, expr)?;
            }
            StmtKind::Return { expr } => {
                self.generate_stmt_return(f, expr)?;
            }
            StmtKind::If {
                condition_expr,
                then_stmt,
                else_stmt,
            } => {
                self.generate_stmt_if(f, condition_expr, *then_stmt, *else_stmt)?;
            }
            StmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt,
            } => {
                self.generate_stmt_for(f, init_expr, condition_expr, delta_expr, *run_stmt)?;
            }
            StmtKind::While {
                condition_expr,
                run_stmt,
            } => {
                self.generate_stmt_while(f, condition_expr, *run_stmt)?;
            }
            StmtKind::Cpd { stmts } => {
                self.generate_stmt_cpd(f, stmts)?;
            }
        }
        Ok(())
    }

    fn generate_stmt_return(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Option<Expr>,
    ) -> anyhow::Result<()> {
        if let Some(expr) = expr {
            self.generate_expr(f, expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }
        writeln!(f, "\tjmp .Lmain_ret")?;
        Ok(())
    }

    fn generate_stmt_expr(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Option<Expr>,
    ) -> anyhow::Result<()> {
        if let Some(expr) = expr {
            self.generate_expr(f, expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }
        Ok(())
    }

    fn generate_stmt_if(
        &mut self,
        f: &mut BufWriter<File>,
        condition_expr: Expr,
        then_stmt: Stmt,
        else_stmt: Option<Stmt>,
    ) -> anyhow::Result<()> {
        let label_num = self.label_num();
        self.generate_expr(f, condition_expr)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tcmp {}, 0", Reg::Rax.qword())?;
        writeln!(f, "\tje .Lelse{}", label_num)?;
        self.generate_stmt(f, then_stmt)?;
        writeln!(f, "\tjmp .Lend{}", label_num)?;
        writeln!(f, ".Lelse{}:", label_num)?;
        if let Some(else_stmt) = else_stmt {
            self.generate_stmt(f, else_stmt)?;
        }
        writeln!(f, ".Lend{}:", label_num)?;
        Ok(())
    }

    fn generate_stmt_for(
        &mut self,
        f: &mut BufWriter<File>,
        init_expr: Option<Expr>,
        condition_expr: Expr,
        delta_expr: Option<Expr>,
        run_stmt: Stmt,
    ) -> anyhow::Result<()> {
        let label_num = self.label_num();
        if let Some(init_expr) = init_expr {
            self.generate_expr(f, init_expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }
        writeln!(f, ".Lbegin{}:", label_num)?;
        self.generate_expr(f, condition_expr)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tcmp {}, 0", Reg::Rax.qword())?;
        writeln!(f, "\tje .Lend{}", label_num)?;
        self.generate_stmt(f, run_stmt)?;
        if let Some(delta_expr) = delta_expr {
            self.generate_expr(f, delta_expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }
        writeln!(f, "\tjmp .Lbegin{}", label_num)?;
        writeln!(f, ".Lend{}:", label_num)?;
        Ok(())
    }

    fn generate_stmt_while(
        &mut self,
        f: &mut BufWriter<File>,
        condition_expr: Expr,
        run_stmt: Stmt,
    ) -> anyhow::Result<()> {
        let label_num = self.label_num();
        writeln!(f, ".Lbegin{}:", label_num)?;
        self.generate_expr(f, condition_expr)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tcmp {}, 0", Reg::Rax.qword())?;
        writeln!(f, "\tje .Lend{}", label_num)?;
        self.generate_stmt(f, run_stmt)?;
        writeln!(f, "\tjmp .Lbegin{}", label_num)?;
        writeln!(f, ".Lend{}:", label_num)?;
        Ok(())
    }

    fn generate_stmt_cpd(
        &mut self,
        f: &mut BufWriter<File>,
        stmts: Vec<Stmt>,
    ) -> anyhow::Result<()> {
        for stmt in stmts.into_iter() {
            self.generate_stmt(f, stmt)?;
        }
        Ok(())
    }

    fn generate_expr(&mut self, f: &mut BufWriter<File>, expr: Expr) -> anyhow::Result<()> {
        match expr.kind {
            ExprKind::Binary { op_kind, lhs, rhs } => {
                self.generate_expr_binary(f, op_kind, *lhs, *rhs)?;
            }
            ExprKind::Assign { op_kind, lhs, rhs } => {
                self.generate_expr_assign(f, op_kind, *lhs, *rhs)?;
            }
            ExprKind::Comma { lhs, rhs } => {
                self.generate_expr(f, *lhs)?;
                self.generate_pop(f, Reg::Rax)?;
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
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        self.generate_expr_binary_with_reg(f, op_kind, Reg::Rax, Reg::Rdi)?;
        Ok(())
    }

    fn generate_expr_binary_with_reg(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: BinaryOpKind,
        lhs: Reg,
        rhs: Reg,
    ) -> anyhow::Result<()> {
        if lhs != Reg::Rax {
            writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), lhs.qword())?;
        }
        if rhs != Reg::Rdi {
            writeln!(f, "\tmov {}, {}", Reg::Rdi.qword(), rhs.qword())?;
        }
        match op_kind {
            BinaryOpKind::Add => {
                writeln!(f, "\tadd {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::Sub => {
                writeln!(f, "\tsub {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::Mul => {
                writeln!(f, "\timul {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::Div => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Reg::Rdi.qword())?;
            }
            BinaryOpKind::Rem => {
                writeln!(f, "\tcqo")?;
                writeln!(f, "\tidiv {}", Reg::Rdi.qword())?;
                writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), Reg::Rdx.qword())?;
            }
            BinaryOpKind::BitAnd => {
                writeln!(f, "\tand {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::BitOr => {
                writeln!(f, "\tor {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::BitXor => {
                writeln!(f, "\txor {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            }
            BinaryOpKind::LShift => {
                writeln!(f, "\tmov {}, {}", Reg::Rcx.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tshl {}, {}", Reg::Rax.qword(), Reg::Rcx.byte())?;
            }
            BinaryOpKind::RShift => {
                writeln!(f, "\tmov {}, {}", Reg::Rcx.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tshr {}, {}", Reg::Rax.qword(), Reg::Rcx.byte())?;
            }
            BinaryOpKind::Lt => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetl {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            BinaryOpKind::LtEqual => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetle {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            BinaryOpKind::Eq => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsete {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
            BinaryOpKind::Ne => {
                writeln!(f, "\tcmp {}, {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
                writeln!(f, "\tsetne {}", Reg::Rax.byte())?;
                writeln!(f, "\tmovzb {}, {}", Reg::Rax.qword(), Reg::Rax.byte())?;
            }
        }
        self.generate_push_with_reg(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_assign(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: BinaryOpKind,
        lhs: Expr,
        rhs: Expr,
    ) -> anyhow::Result<()> {
        if op_kind == BinaryOpKind::Eq {
            self.generate_expr_left_var(f, lhs)?;
            self.generate_expr(f, rhs)?;
            self.generate_pop(f, Reg::Rdi)?;
            self.generate_pop(f, Reg::Rax)?;
            writeln!(f, "\tmov {}, [{}]", Reg::R8.qword(), Reg::Rax.qword())?;
            self.generate_push_with_reg(f, Reg::Rax)?;
            self.generate_expr_binary_with_reg(f, op_kind, Reg::R8, Reg::Rdi)?;
            self.generate_pop(f, Reg::Rdi)?;
            self.generate_pop(f, Reg::Rax)?;
            writeln!(f, "\tmov [{}], {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            self.generate_push_with_reg(f, Reg::Rdi)?;
        } else {
            self.generate_expr_left_var(f, lhs)?;
            self.generate_expr(f, rhs)?;
            self.generate_pop(f, Reg::Rdi)?;
            self.generate_pop(f, Reg::Rax)?;
            writeln!(f, "\tmov [{}], {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
            self.generate_push_with_reg(f, Reg::Rdi)?;
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
        expr: Expr,
    ) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Reg::Rdi)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push_with_reg(f, Reg::Rax)?;
        writeln!(f, "\tadd {}, 1", Reg::Rax.qword())?;
        writeln!(f, "\tmov [{}], {}", Reg::Rdi.qword(), Reg::Rax.qword())?;
        Ok(())
    }

    fn generate_expr_postfix_decrement(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Expr,
    ) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Reg::Rdi)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push_with_reg(f, Reg::Rax)?;
        writeln!(f, "\tsub {}, 1", Reg::Rax.qword())?;
        writeln!(f, "\tmov [{}], {}", Reg::Rdi.qword(), Reg::Rax.qword())?;
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
            writeln!(f, "\tsub {}, 8", Reg::Rsp.qword())?;
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
                self.generate_pop(f, Reg::R10)?;
                self.generate_set_func_args(f, arg_len)?;
                writeln!(f, "\tcall {}", Reg::R10.qword())?;
            }
        };
        if stack_adjust {
            writeln!(f, "\tadd {}, {}", Reg::Rsp.qword(), (stack + 1) * 8)?;
            self.stack -= stack + 1;
        } else {
            writeln!(f, "\tadd {}, {}", Reg::Rsp.qword(), stack * 8)?;
            self.stack -= stack;
        }
        self.generate_push_with_reg(f, Reg::Rax)?;
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
        let regs = vec![Reg::Rdi, Reg::Rsi, Reg::Rdx, Reg::Rcx, Reg::R8, Reg::R9];
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
                writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), Reg::Rbp.qword())?;
                writeln!(f, "\tsub {}, {}", Reg::Rax.qword(), var.offset)?;
            }
            _ => {
                return Err(anyhow!(
                    "{}Must be a changeable left-hand side value",
                    expr.position
                ))
            }
        }
        self.generate_push_with_reg(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_var(&mut self, f: &mut BufWriter<File>, expr: Expr) -> anyhow::Result<()> {
        self.generate_expr_left_var(f, expr)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov {}, [{}]", Reg::Rax.qword(), Reg::Rax.qword())?;
        self.generate_push_with_reg(f, Reg::Rax)?;
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

    fn generate_push_with_reg(&mut self, f: &mut BufWriter<File>, reg: Reg) -> anyhow::Result<()> {
        writeln!(f, "\tpush {}", reg.qword())?;
        self.stack += 1;
        Ok(())
    }

    fn generate_push_with_num(&mut self, f: &mut BufWriter<File>, num: i32) -> anyhow::Result<()> {
        writeln!(f, "\tpush {}", num)?;
        self.stack += 1;
        Ok(())
    }

    fn generate_pop(&mut self, f: &mut BufWriter<File>, reg: Reg) -> anyhow::Result<()> {
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
