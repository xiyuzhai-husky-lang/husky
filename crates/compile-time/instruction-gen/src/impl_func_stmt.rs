use crate::*;
use vm::{EagerContract, InitKind, Instruction, InstructionKind};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_func_stmts(&mut self, stmts: &[Arc<FuncStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_func_stmt(stmt.clone()));
    }

    fn compile_func_stmt(&mut self, stmt: Arc<FuncStmt>) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.compile_expr(initial_value);
                self.def_variable(varname.ident)
            }
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => {
                self.compile_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            FuncStmtVariant::Branches { .. } => todo!(),
        }
    }
}
