use std::sync::Arc;

use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, DeclStmt, DeclStmtKind, Expr};

pub fn gen_decl_stmt_instructions(stmts: &[Arc<DeclStmt>]) -> Vec<Instruction> {
    let mut generator = DeclStmtInstructionGenerator::new();
    stmts
        .iter()
        .for_each(|stmt| generator.gen_stmt_instructions(stmt));
    generator.instructions
}

struct DeclStmtInstructionGenerator {
    instructions: Vec<Instruction>,
}

impl DeclStmtInstructionGenerator {
    fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }

    fn gen_stmt_instructions(&mut self, stmt: &DeclStmt) {
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
            DeclStmtKind::Branch {} => todo!(),
        }
    }
}

impl ExprInstructionBuilder for DeclStmtInstructionGenerator {
    fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}
