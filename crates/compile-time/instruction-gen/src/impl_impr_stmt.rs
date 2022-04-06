use crate::*;

use vm::{EagerContract, Instruction, InstructionKind, VMLoopKind};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_impr_stmts(&mut self, stmts: &[Arc<ProcStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_impr_stmt(stmt.clone()));
    }

    fn compile_impr_stmt(&mut self, stmt: Arc<ProcStmt>) {
        match stmt.kind {
            ProcStmtKind::Init {
                varname,
                ref initial_value,
                init_kind,
                ..
            } => {
                self.compile_expr(initial_value);
                self.def_variable(varname, init_kind, stmt)
            }
            ProcStmtKind::Assert { ref condition } => todo!(),
            ProcStmtKind::Return { ref result } => {
                self.compile_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            ProcStmtKind::Execute { ref expr } => {
                self.compile_expr(expr);
            }
            ProcStmtKind::BranchGroup { .. } => todo!(),
            ProcStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => self.compile_loop(loop_kind, stmt.clone(), stmts),
        }
    }

    fn compile_loop(
        &mut self,
        loop_kind: &LoopKind,
        loop_stmt: Arc<ProcStmt>,
        body_stmts: &[Arc<ProcStmt>],
    ) {
        match loop_kind {
            LoopKind::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => {
                self.compile_boundary(initial_boundary, &loop_stmt);
                self.compile_boundary(final_boundary, &loop_stmt);
                let body = self.build_impr_block(body_stmts);
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: loop_kind.into(),
                    },
                    loop_stmt,
                ));
            }
            LoopKind::ForExt {
                frame_var,
                frame_varidx,
                final_boundary,
                step,
            } => {
                self.compile_boundary(final_boundary, &loop_stmt);
                let body = self.build_impr_block(body_stmts);
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: loop_kind.into(),
                    },
                    loop_stmt,
                ));
            }
            LoopKind::While { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionKind::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                block_sheet_builder.compile_impr_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: loop_kind.into(),
                    },
                    loop_stmt,
                ));
            }
            LoopKind::DoWhile { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_impr_stmts(body_stmts);
                block_sheet_builder.compile_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionKind::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: loop_kind.into(),
                    },
                    loop_stmt,
                ));
            }
        }
    }

    fn compile_boundary(&mut self, boundary: &Boundary, loop_stmt: &Arc<ProcStmt>) {
        if let Some(ref bound) = boundary.opt_bound {
            self.compile_expr(bound)
        } else {
            self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(0i32.into()),
                loop_stmt.clone(),
            ))
        }
    }
}
