use virtual_stack::Instruction;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Vec<LazyStmt>,
    pub instructions: Vec<Instruction>,
}

impl DatasetConfig {
    pub fn new(stmts: Vec<LazyStmt>) -> Self {
        let instructions = build_instructions(&stmts);
        Self {
            stmts,
            instructions,
        }
    }
}

fn build_instructions(stmts: &[LazyStmt]) -> Vec<Instruction> {
    stmts.iter().map(|stmt| build_instruction(stmt)).collect()
}

fn build_instruction(stmt: &LazyStmt) -> Instruction {
    match &stmt.kind {
        LazyStmtKind::Init {
            varname,
            initial_value,
        } => todo!(),
        LazyStmtKind::Assert { condition } => todo!(),
        LazyStmtKind::Return { result } => todo!(),
    }
}
