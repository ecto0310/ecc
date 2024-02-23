use std::{fs::File, io::BufWriter, io::Write};

use crate::{
    analyze::{
        expr::Expr,
        stmt::{Stmt, StmtKind},
    },
    generate::register::Register,
};

use super::Generator;

impl Generator {
    pub fn generate_stmt(&mut self, f: &mut BufWriter<File>, stmt: Stmt) -> anyhow::Result<()> {
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
            self.generate_pop(f, Register::Rax)?;
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
            self.generate_pop(f, Register::Rax)?;
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
        self.generate_pop(f, Register::Rax)?;
        writeln!(f, "\tcmp {}, 0", Register::Rax.qword())?;
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
            self.generate_pop(f, Register::Rax)?;
        }
        writeln!(f, ".Lbegin{}:", label_num)?;
        self.generate_expr(f, condition_expr)?;
        self.generate_pop(f, Register::Rax)?;
        writeln!(f, "\tcmp {}, 0", Register::Rax.qword())?;
        writeln!(f, "\tje .Lend{}", label_num)?;
        self.generate_stmt(f, run_stmt)?;
        if let Some(delta_expr) = delta_expr {
            self.generate_expr(f, delta_expr)?;
            self.generate_pop(f, Register::Rax)?;
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
        self.generate_pop(f, Register::Rax)?;
        writeln!(f, "\tcmp {}, 0", Register::Rax.qword())?;
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
}
