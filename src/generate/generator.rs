use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::analyze::gen_expr::GenExpr;
use crate::analyze::gen_expr_kind::{GenBinaryOpKind, GenExprKind, GenFuncCallKind};
use crate::analyze::gen_stmt::GenStmt;
use crate::analyze::gen_stmt_kind::GenStmtKind;
use crate::{analyze::gen_tree::GenTree, error::Error};

use super::reg::Reg;

pub struct Generator {
    label: usize,
    stack: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self { label: 0, stack: 0 }
    }

    pub fn generate(&mut self, f: &mut BufWriter<File>, gen_tree: GenTree) -> Result<(), Error> {
        writeln!(f, ".intel_syntax noprefix")?;
        writeln!(f, ".globl main")?;
        writeln!(f, "main:")?;
        self.generate_push_with_reg(f, Reg::Rbp)?;
        writeln!(f, "\tmov {}, {}", Reg::Rbp.qword(), Reg::Rsp.qword())?;
        let offset = if gen_tree.offset % 16 == 0 {
            gen_tree.offset
        } else {
            gen_tree.offset + gen_tree.offset % 16
        };
        writeln!(f, "\tsub {}, {}", Reg::Rsp.qword(), offset)?;

        for stmt in gen_tree.stmts.into_iter() {
            self.generate_stmt(f, stmt)?;
        }

        writeln!(f, ".Lmain_ret:")?;
        writeln!(f, "\tmov {}, {}", Reg::Rsp.qword(), Reg::Rbp.qword())?;
        self.generate_pop(f, Reg::Rbp)?;
        writeln!(f, "\tret")?;
        Ok(())
    }

    fn generate_stmt(&mut self, f: &mut BufWriter<File>, stmt: GenStmt) -> Result<(), Error> {
        match stmt.kind {
            GenStmtKind::Expr { expr } => {
                self.generate_stmt_expr(f, expr)?;
            }
            GenStmtKind::Return { expr } => {
                self.generate_stmt_return(f, expr)?;
            }
            GenStmtKind::If {
                condition_expr,
                then_stmt,
                else_stmt,
            } => {
                self.generate_stmt_if(f, condition_expr, *then_stmt, *else_stmt)?;
            }
            GenStmtKind::For {
                init_expr,
                condition_expr,
                delta_expr,
                run_stmt,
            } => {
                self.generate_stmt_for(f, init_expr, condition_expr, delta_expr, *run_stmt)?;
            }
            GenStmtKind::While {
                condition_expr,
                run_stmt,
            } => {
                self.generate_stmt_while(f, condition_expr, *run_stmt)?;
            }
            GenStmtKind::Cpd { stmts } => {
                self.generate_stmt_cpd(f, stmts)?;
            }
        }
        Ok(())
    }

    fn generate_stmt_return(
        &mut self,
        f: &mut BufWriter<File>,
        expr: Option<GenExpr>,
    ) -> Result<(), Error> {
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
        expr: Option<GenExpr>,
    ) -> Result<(), Error> {
        if let Some(expr) = expr {
            self.generate_expr(f, expr)?;
            self.generate_pop(f, Reg::Rax)?;
        }
        Ok(())
    }

    fn generate_stmt_if(
        &mut self,
        f: &mut BufWriter<File>,
        condition_expr: GenExpr,
        then_stmt: GenStmt,
        else_stmt: Option<GenStmt>,
    ) -> Result<(), Error> {
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
        init_expr: Option<GenExpr>,
        condition_expr: GenExpr,
        delta_expr: Option<GenExpr>,
        run_stmt: GenStmt,
    ) -> Result<(), Error> {
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
        condition_expr: GenExpr,
        run_stmt: GenStmt,
    ) -> Result<(), Error> {
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
        stmts: Vec<GenStmt>,
    ) -> Result<(), Error> {
        for stmt in stmts.into_iter() {
            self.generate_stmt(f, stmt)?;
        }
        Ok(())
    }

    fn generate_expr(&mut self, f: &mut BufWriter<File>, expr: GenExpr) -> Result<(), Error> {
        match expr.kind {
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
            GenExprKind::Var { .. } => {
                self.generate_expr_var(f, expr)?;
            }
            GenExprKind::Number { number } => {
                self.generate_expr_number(f, number)?;
            }
            GenExprKind::Func { name, args } => self.generate_expr_func(f, name, args)?,
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
        self.generate_push_with_reg(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_assign(
        &mut self,
        f: &mut BufWriter<File>,
        lhs: GenExpr,
        rhs: GenExpr,
    ) -> Result<(), Error> {
        self.generate_expr_left_var(f, lhs)?;
        self.generate_expr(f, rhs)?;
        self.generate_pop(f, Reg::Rdi)?;
        self.generate_pop(f, Reg::Rax)?;
        writeln!(f, "\tmov [{}], {}", Reg::Rax.qword(), Reg::Rdi.qword())?;
        self.generate_push_with_reg(f, Reg::Rdi)?;
        Ok(())
    }

    fn generate_expr_assign_op(
        &mut self,
        f: &mut BufWriter<File>,
        op_kind: GenBinaryOpKind,
        lhs: GenExpr,
        rhs: GenExpr,
    ) -> Result<(), Error> {
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
        expr: GenExpr,
    ) -> Result<(), Error> {
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
        expr: GenExpr,
    ) -> Result<(), Error> {
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
        name: GenFuncCallKind,
        args: Vec<GenExpr>,
    ) -> Result<(), Error> {
        let arg_len = args.len();
        let stack: usize = if 6 < arg_len { arg_len - 6 } else { 0 };
        let stack_adjust = (self.stack + stack) % 2 == 1;
        if stack_adjust {
            writeln!(f, "\tsub {}, 8", Reg::Rsp.qword())?;
            self.stack += 1;
        }
        self.generate_expr_func_args(f, args)?;
        match name {
            GenFuncCallKind::Label { name } => {
                self.generate_set_func_args(f, arg_len)?;
                writeln!(f, "\tcall {}", name)?;
            }
            GenFuncCallKind::Expr { expr } => {
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
        args: Vec<GenExpr>,
    ) -> Result<(), Error> {
        for arg in args.into_iter().rev() {
            self.generate_expr(f, arg)?;
        }
        Ok(())
    }

    fn generate_set_func_args(
        &mut self,
        f: &mut BufWriter<File>,
        length: usize,
    ) -> Result<(), Error> {
        let regs = vec![Reg::Rdi, Reg::Rsi, Reg::Rdx, Reg::Rcx, Reg::R8, Reg::R9];
        for reg in regs.iter().take(std::cmp::min(length, 6)) {
            self.generate_pop(f, reg.clone())?;
        }
        Ok(())
    }

    fn generate_expr_left_var(
        &mut self,
        f: &mut BufWriter<File>,
        expr: GenExpr,
    ) -> Result<(), Error> {
        match expr.kind {
            GenExprKind::Var { var } => {
                writeln!(f, "\tmov {}, {}", Reg::Rax.qword(), Reg::Rbp.qword())?;
                writeln!(f, "\tsub {}, {}", Reg::Rax.qword(), var.offset)?;
            }
            _ => return Err(Error::new_unexpected()),
        }
        self.generate_push_with_reg(f, Reg::Rax)?;
        Ok(())
    }

    fn generate_expr_var(&mut self, f: &mut BufWriter<File>, expr: GenExpr) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
        let number = number as i32;
        self.generate_push_with_num(f, number)?;
        Ok(())
    }

    fn generate_push_with_reg(&mut self, f: &mut BufWriter<File>, reg: Reg) -> Result<(), Error> {
        writeln!(f, "\tpush {}", reg.qword())?;
        self.stack += 1;
        Ok(())
    }

    fn generate_push_with_num(&mut self, f: &mut BufWriter<File>, num: i32) -> Result<(), Error> {
        writeln!(f, "\tpush {}", num)?;
        self.stack += 1;
        Ok(())
    }

    fn generate_pop(&mut self, f: &mut BufWriter<File>, reg: Reg) -> Result<(), Error> {
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
