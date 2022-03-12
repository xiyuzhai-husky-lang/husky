use crate::*;

use vm::{Contract, Instruction, InstructionKind, VMLoopKind};

impl InstructionSheetBuilder {
    pub(super) fn compile_impr_stmts(&mut self, stmts: &[Arc<ImprStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_impr_stmt(stmt.clone()));
    }

    fn compile_impr_stmt(&mut self, stmt: Arc<ImprStmt>) {
        match stmt.kind {
            ImprStmtKind::Init {
                varname,
                ref initial_value,
                init_kind,
                ..
            } => {
                self.compile_expr(initial_value);
                self.def_variable(varname, init_kind, stmt)
            }
            ImprStmtKind::Assert { ref condition } => todo!(),
            ImprStmtKind::Return { ref result } => {
                self.compile_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            ImprStmtKind::Execute { ref expr } => {
                self.compile_expr(expr);
            }
            ImprStmtKind::BranchGroup { .. } => todo!(),
            ImprStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => self.compile_loop(loop_kind, stmt.clone(), stmts),
        }
    }

    fn compile_loop(
        &mut self,
        loop_kind: &LoopKind,
        loop_stmt: Arc<ImprStmt>,
        body_stmts: &[Arc<ImprStmt>],
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
                let mut block_sheet_builder = self.subsheet_builder();
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
            LoopKind::ForExt => todo!(),
            LoopKind::While => todo!(),
            LoopKind::DoWhile => todo!(),
        }
    }

    fn compile_boundary(&mut self, boundary: &Boundary, loop_stmt: &Arc<ImprStmt>) {
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
