mod impl_basic;
mod impl_decl_stmt;
mod impl_expr;
mod impl_impr_stmt;

use std::sync::Arc;

use vm::InstructionSheet;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionSheetBuilder {
    sheet: InstructionSheet,
}

impl InstructionSheetBuilder {
    pub fn new_decl(
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<FuncStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(inputs, has_this);
        builder.compile_decl_stmts(stmts);
        builder.finalize()
    }

    pub fn new_impr(
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<ProcStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(inputs, has_this);
        builder.compile_impr_stmts(stmts);
        builder.finalize()
    }

    fn new(inputs: Vec<CustomIdentifier>, has_this: bool) -> Self {
        Self {
            sheet: InstructionSheet::new(inputs, has_this),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            sheet: self.sheet.init_subsheet(),
        }
    }

    fn build_impr_block(&self, stmts: &[Arc<ProcStmt>]) -> Arc<InstructionSheet> {
        let mut block_sheet_builder = self.subsheet_builder();
        block_sheet_builder.compile_impr_stmts(stmts);
        block_sheet_builder.finalize()
    }

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
