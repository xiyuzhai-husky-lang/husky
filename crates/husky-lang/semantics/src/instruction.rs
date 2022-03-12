mod impl_basic;
mod impl_decl_stmt;
mod impl_impr_stmt;
mod impl_strict_expr;

use vm::{InitKind, Instruction, InstructionKind, InstructionSheet, VariableStack};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionSheetBuilder {
    sheet: InstructionSheet,
}

impl InstructionSheetBuilder {
    pub fn new_decl(stmts: &[Arc<DeclStmt>]) -> Arc<InstructionSheet> {
        let mut builder = Self::new();
        builder.compile_decl_stmts(stmts);
        builder.finalize()
    }

    pub fn new_impr(stmts: &[Arc<ImprStmt>]) -> Arc<InstructionSheet> {
        let mut builder = Self::new();
        builder.compile_impr_stmts(stmts);
        builder.finalize()
    }

    fn new() -> Self {
        Self {
            sheet: Default::default(),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            sheet: self.sheet.init_subsheet(),
        }
    }

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
