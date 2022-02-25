use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, stmt::gen_decl_stmt_instructions, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Arc<Vec<Arc<DeclStmt>>>,
    pub instruction_sheet: InstructionSheet,
}

impl DatasetConfig {
    pub fn new(stmts: Arc<Vec<Arc<DeclStmt>>>) -> Self {
        let mut instruction_sheet = InstructionSheet::default();

        gen_decl_stmt_instructions(&stmts, &mut instruction_sheet);
        Self {
            stmts,
            instruction_sheet,
        }
    }
}
