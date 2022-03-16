mod impl_basic;
mod impl_eager_decl_stmt;
mod impl_eager_expr;
mod impl_eager_impr_stmt;

use vm::{InitKind, Instruction, InstructionKind, InstructionSheet, VariableStack};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionSheetBuilder {
    sheet: InstructionSheet,
}

impl InstructionSheetBuilder {
    pub fn new_decl(
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<DeclStmt>],
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(inputs);
        builder.compile_decl_stmts(stmts);
        builder.finalize()
    }

    pub fn new_impr(
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<ImprStmt>],
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(inputs);
        builder.compile_impr_stmts(stmts);
        builder.finalize()
    }

    fn new(inputs: Vec<CustomIdentifier>) -> Self {
        Self {
            sheet: InstructionSheet::new(inputs),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            sheet: self.sheet.init_subsheet(),
        }
    }

    fn build_impr_block(&self, stmts: &[Arc<ImprStmt>]) -> Arc<InstructionSheet> {
        let mut block_sheet_builder = self.subsheet_builder();
        block_sheet_builder.compile_impr_stmts(stmts);
        block_sheet_builder.finalize()
    }

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
