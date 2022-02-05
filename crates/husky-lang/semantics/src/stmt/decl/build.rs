use std::sync::Arc;

use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, DeclStmt, DeclStmtKind, Expr};

pub fn build_decl_stmt_instructions(stmts: &[Arc<DeclStmt>]) -> Vec<Instruction> {
    let mut builder = DeclStmtInstructionBuilder::new();
    stmts
        .iter()
        .for_each(|stmt| builder.build_stmt_instructions(stmt));
    builder.instructions
}

struct DeclStmtInstructionBuilder {
    instructions: Vec<Instruction>,
}

impl DeclStmtInstructionBuilder {
    fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }

    fn build_stmt_instructions(&mut self, stmt: &DeclStmt) {
        match stmt.kind {
            DeclStmtKind::Init {
                varname,
                value: ref initial_value,
            } => todo!(),
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => {
                self.build_expr_instructions(result);
                self.push_instruction(Instruction {
                    kind: InstructionKind::Return,
                });
            }
        }
    }
}

impl ExprInstructionBuilder for DeclStmtInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}
