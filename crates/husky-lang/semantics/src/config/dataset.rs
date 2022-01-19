use virtual_stack::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, stmt::build_decl_stmt_instructions, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Vec<DeclStmt>,
    pub instructions: Vec<Instruction>,
}

impl DatasetConfig {
    pub fn new(stmts: Vec<DeclStmt>) -> Self {
        let instructions = build_decl_stmt_instructions(&stmts);
        Self {
            stmts,
            instructions,
        }
    }
}
