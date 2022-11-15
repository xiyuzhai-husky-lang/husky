mod context;
mod impl_basic;
mod impl_eager_expr;
mod impl_func_stmt;
mod impl_proc_stmt;
mod query;

pub use query::*;

use context::*;
use fold::LocalValue;

use husky_eager_semantics::*;
use husky_entity_semantics::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_term::*;
use husky_vm::InstructionSheet;
use husky_word::*;
use std::sync::Arc;

pub fn new_visual_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    stmts: &[Arc<FuncStmt>],
) -> Arc<InstructionSheet> {
    let mut builder = InstructionSheetBuilder::new(db, [].into_iter(), true);
    builder.compile_func_stmts(stmts);
    builder.finalize()
}

pub fn new_func_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    inputs: impl Iterator<Item = CustomIdentifier>,
    stmts: &[Arc<FuncStmt>],
    has_this: bool,
) -> Arc<InstructionSheet> {
    let mut builder = InstructionSheetBuilder::new(db, inputs, has_this);
    builder.compile_func_stmts(stmts);
    builder.finalize()
}

pub fn new_proc_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    inputs: impl Iterator<Item = CustomIdentifier>,
    stmts: &[Arc<ProcStmt>],
    has_this: bool,
) -> Arc<InstructionSheet> {
    let mut builder = InstructionSheetBuilder::new(db, inputs, has_this);
    builder.compile_proc_stmts(stmts);
    builder.finalize()
}

struct InstructionSheetBuilder<'a> {
    db: &'a dyn InstructionGenQueryGroup,
    sheet: InstructionSheet,
    context: LocalValue<InstructionGenContext>,
}

impl<'a> InstructionSheetBuilder<'a> {
    fn new(
        db: &'a dyn InstructionGenQueryGroup,
        inputs: impl Iterator<Item = CustomIdentifier>,
        has_this: bool,
    ) -> Self {
        Self {
            db,
            sheet: InstructionSheet::new(inputs, has_this),
            context: LocalValue::new(InstructionGenContext::Normal),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            db: self.db,
            sheet: self.sheet.init_subsheet(),
            context: LocalValue::new(InstructionGenContext::Normal),
        }
    }

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
