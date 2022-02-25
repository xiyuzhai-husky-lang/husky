use std::sync::Arc;

use vm::{Instruction, InstructionKind};

use crate::{expr::ExprInstructionBuilder, *};

pub fn gen_decl_stmt_instructions(stmts: &[Arc<DeclStmt>], sheet: &mut InstructionSheet) {
    let mut generator = DeclStmtInstructionGenerator::new(sheet);
    stmts
        .iter()
        .for_each(|stmt| generator.gen_stmt_instructions(stmt));
}

struct DeclStmtInstructionGenerator<'a> {
    sheet: &'a mut InstructionSheet,
}

impl<'a> DeclStmtInstructionGenerator<'a> {
    fn new(sheet: &'a mut InstructionSheet) -> Self {
        Self { sheet }
    }

    fn gen_stmt_instructions(&mut self, stmt: &DeclStmt) {
        match stmt.kind {
            DeclStmtKind::Init {
                varname,
                value: ref initial_value,
            } => todo!(),
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => {
                self.gen_expr_instructions(result);
                self.push_instruction(Instruction {
                    kind: InstructionKind::Return,
                });
            }
            DeclStmtKind::Branches { .. } => todo!(),
        }
    }
}

impl<'a> ExprInstructionBuilder for DeclStmtInstructionGenerator<'a> {
    fn push_instruction(&mut self, instruction: Instruction) {
        self.sheet.push_instruction(instruction)
    }
}
