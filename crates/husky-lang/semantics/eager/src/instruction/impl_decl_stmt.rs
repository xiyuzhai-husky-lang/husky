use crate::*;

use vm::{Contract, Instruction, InstructionKind};

impl InstructionSheetBuilder {
    pub(super) fn compile_decl_stmts(&mut self, stmts: &[Arc<DeclStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_decl_stmt(stmt.clone()));
    }

    fn compile_decl_stmt(&mut self, stmt: Arc<DeclStmt>) {
        match stmt.kind {
            DeclStmtKind::Init {
                varname,
                value: ref initial_value,
            } => todo!(),
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => {
                self.compile_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            DeclStmtKind::Branches { .. } => todo!(),
        }
    }
}
