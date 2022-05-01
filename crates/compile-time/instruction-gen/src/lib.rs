mod impl_basic;
mod impl_expr;
mod impl_func_stmt;
mod impl_proc_stmt;
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
    pub fn new_func(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: impl Iterator<Item = CustomIdentifier>,
        stmts: &[Arc<FuncStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(db, inputs, has_this);
        builder.compile_func_stmts(stmts);
        builder.finalize()
    }

    pub fn new_impr(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: impl Iterator<Item = CustomIdentifier>,
        stmts: &[Arc<ProcStmt>],
        has_this: bool,
    ) -> Arc<InstructionSheet> {
        let mut builder = Self::new(db, inputs, has_this);
        builder.compile_proc_stmts(stmts);
        builder.finalize()
    }

    fn new(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: impl Iterator<Item = CustomIdentifier>,
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

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
