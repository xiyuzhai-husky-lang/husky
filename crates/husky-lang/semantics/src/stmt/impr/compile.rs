use std::sync::Arc;

use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, *};

pub fn gen_impr_stmt_instructions(stmts: &[Arc<ImprStmt>], sheet: &mut InstructionSheet) {
    let mut generator = ImprStmtInstructionGenerator::new(sheet);
    stmts
        .iter()
        .for_each(|stmt| generator.gen_stmt_instructions(stmt));
}

struct ImprStmtInstructionGenerator<'a> {
    sheet: &'a mut InstructionSheet,
}

impl<'a> ImprStmtInstructionGenerator<'a> {
    fn new(sheet: &'a mut InstructionSheet) -> Self {
        Self { sheet }
    }

    fn gen_stmt_instructions(&mut self, stmt: &ImprStmt) {
        match stmt.kind {
            ImprStmtKind::Init {
                varname,
                ref initial_value,
            } => todo!(),
            ImprStmtKind::Assert { ref condition } => todo!(),
            ImprStmtKind::Return { ref result } => {
                self.gen_expr_instructions(result);
                self.push_instruction(Instruction {
                    kind: InstructionKind::Return,
                });
            }
            ImprStmtKind::BranchGroup { .. } => todo!(),
            ImprStmtKind::Loop => todo!(),
        }
    }
}

impl<'a> ExprInstructionBuilder for ImprStmtInstructionGenerator<'a> {
    fn push_instruction(&mut self, instruction: Instruction) {
        self.sheet.push_instruction(instruction)
    }
}
