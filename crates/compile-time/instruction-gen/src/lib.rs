mod impl_basic;
mod impl_eager_expr;
mod impl_func_stmt;
mod impl_proc_stmt;
mod impl_xml_expr;
mod query;

pub use query::*;

use entity_route::*;
use print_utils::*;
use semantics_eager::*;
use semantics_entity::*;
use std::sync::Arc;
use vm::{Instruction, InstructionSheet};
use word::*;

pub fn new_visual_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    stmts: &[Arc<FuncStmt>],
) -> Arc<InstructionSheet> {
    let mut builder = InstructionSheetBuilder::new(db, [].into_iter(), true);
    builder.compile_func_stmts(stmts);
    builder.finalize()
}

fn new_func_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    inputs: impl Iterator<Item = CustomIdentifier>,
    stmts: &[Arc<FuncStmt>],
    has_this: bool,
) -> Arc<InstructionSheet> {
    let mut builder = InstructionSheetBuilder::new(db, inputs, has_this);
    builder.compile_func_stmts(stmts);
    builder.finalize()
}

fn new_impr_instruction_sheet(
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
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            db: self.db,
            sheet: self.sheet.init_subsheet(),
        }
    }

    fn eject_instructions(&mut self) -> Vec<Instruction> {
        std::mem::take(&mut self.sheet.instructions)
    }

    fn finalize(self) -> Arc<InstructionSheet> {
        Arc::new(self.sheet)
    }
}
