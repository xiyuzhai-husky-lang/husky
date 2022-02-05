use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, stmt::build_decl_stmt_instructions, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Vec<Arc<DeclStmt>>,
    pub instructions: Vec<Instruction>,
}

impl DatasetConfig {
    pub fn new(stmts: Vec<Arc<DeclStmt>>) -> Self {
        let instructions = build_decl_stmt_instructions(&stmts);
        Self {
            stmts,
            instructions,
        }
    }
}
