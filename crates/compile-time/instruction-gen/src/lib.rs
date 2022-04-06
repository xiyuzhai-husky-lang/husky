mod impl_basic;
mod impl_decl_stmt;
mod impl_expr;
mod impl_impr_stmt;
mod query;

pub use query::*;

use entity_route::*;
use print_utils::*;
use semantics_eager::*;
use semantics_entity::*;
use std::sync::Arc;
use vm::InstructionSheet;
use word::*;

pub struct InstructionSheetBuilder<'a> {
    db: &'a dyn InstructionGenQueryGroup,
    sheet: InstructionSheet,
}

impl<'a> InstructionSheetBuilder<'a> {
    pub fn new_decl(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<FuncStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(db, inputs, has_this);
        builder.compile_decl_stmts(stmts);
        builder.finalize()
    }

    pub fn new_impr(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: Vec<CustomIdentifier>,
        stmts: &[Arc<ProcStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(db, inputs, has_this);
        builder.compile_impr_stmts(stmts);
        builder.finalize()
    }

    fn new(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: Vec<CustomIdentifier>,
        has_this: bool,
    ) -> Self {
        Self {
            db,
            sheet: InstructionSheet::new(inputs, has_this),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            db: self.db,
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
