// mod impl_basic;
// mod impl_eager_expr;
// mod impl_func_stmt;
// mod impl_proc_stmt;

use crate::*;

// pub fn new_visual_instruction_sheet(
//     db: &dyn InstructionDb,
//     stmts: &[Arc<FuncStmt>],
// ) -> InstructionRegion {
//     let mut builder = InstructionSheetBuilder::new(db, [].into_iter(), true);
//     builder.compile_func_stmts(stmts);
//     builder.finalize()
// }

// pub fn new_func_instruction_sheet(
//     db: &dyn InstructionDb,
//     inputs: impl Iterator<Item = Ident>,
//     stmts: &[Arc<FuncStmt>],
//     has_this: bool,
// ) -> InstructionRegion {
//     let mut builder = InstructionSheetBuilder::new(db, inputs, has_this);
//     builder.compile_func_stmts(stmts);
//     builder.finalize()
// }

// pub fn new_proc_instruction_sheet(
//     db: &dyn InstructionDb,
//     inputs: impl Iterator<Item = Ident>,
//     stmts: &[Arc<ProcStmt>],
//     has_this: bool,
// ) -> InstructionRegion {
//     let mut builder = InstructionSheetBuilder::new(db, inputs, has_this);
//     builder.compile_proc_stmts(stmts);
//     builder.finalize()
// }

pub(crate) struct InstructionBuilder<'a> {
    db: &'a dyn InstructionDb,
    // sheet: Instructions,
    // context: LocalValue<InstructionGenContext>,
}

impl<'a> InstructionBuilder<'a> {
    fn new(db: &'a dyn InstructionDb, inputs: impl Iterator<Item = Ident>, has_this: bool) -> Self {
        Self {
            db,
            // sheet: Instructions::new(inputs, has_this),
            // context: LocalValue::new(InstructionGenContext::Normal),
        }
    }

    fn subsheet_builder(&self) -> Self {
        Self {
            db: self.db,
            // sheet: self.sheet.init_subsheet(),
            // context: LocalValue::new(InstructionGenContext::Normal),
        }
    }

    fn finalize(self) -> Instructions {
        todo!()
        // Arc::new(self.sheet)
    }
}
