mod expr;
mod stmt;

use std::collections::{BTreeMap, VecDeque};

use crate::parse::row_program::RowProgram;

use super::{program::Program, stmt::Stmt, variable::Variable};

pub struct Analyzer {
    var: BTreeMap<String, Variable>,
    offset: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            var: BTreeMap::new(),
            offset: 0,
        }
    }

    pub fn analyze(&mut self, row_program: RowProgram) -> anyhow::Result<Program> {
        let stmts = row_program
            .stmts
            .into_iter()
            .map(|stmt| self.analyze_stmt(stmt))
            .collect::<anyhow::Result<VecDeque<Stmt>>>()?;
        Ok(Program::new(stmts, self.offset))
    }

    fn get_var(&mut self, name: String) -> Variable {
        if let Some(var) = self.var.get(&name) {
            var.clone()
        } else {
            self.offset += 8;
            let var = Variable::new(self.offset);
            self.var.insert(name, var.clone());
            var
        }
    }
}
