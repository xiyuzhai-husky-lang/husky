use crate::*;

use avec::Avec;
use vm::{EagerContract, Instruction, InstructionKind, VMBranch, VMLoopKind};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_proc_stmts(&mut self, stmts: &[Arc<ProcStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_proc_stmt(stmt.clone()));
    }

    fn compile_proc_stmt(&mut self, stmt: Arc<ProcStmt>) {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
                ..
            } => {
                self.compile_expr(initial_value);
                self.def_variable(varname.ident)
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.compile_expr(condition);
                self.push_instruction(Instruction::new(InstructionKind::Assert, stmt))
            }
            ProcStmtVariant::Return { ref result } => {
                self.compile_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.compile_expr(expr);
            }
            ProcStmtVariant::BranchGroup { ref branches, .. } => {
                self.push_instruction(Instruction::new(
                    InstructionKind::BranchGroup {
                        branches: self.compile_branch_groups(branches),
                    },
                    stmt,
                ))
            }
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => self.compile_loop(loop_variant, stmt.clone(), stmts),
            ProcStmtVariant::Break => {
                self.push_instruction(Instruction::new(InstructionKind::Break, stmt))
            }
        }
    }

    fn compile_loop(
        &mut self,
        loop_kind: &LoopVariant,
        loop_stmt: Arc<ProcStmt>,
        body_stmts: &[Arc<ProcStmt>],
    ) {
        match loop_kind {
            LoopVariant::For {
                initial_boundary,
                final_boundary,
                frame_var,
                step,
                ..
            } => {
                self.compile_boundary(initial_boundary, &loop_stmt);
                self.compile_boundary(final_boundary, &loop_stmt);
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.def_for_frame_variable(frame_var.ident);
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: VMLoopKind::For {
                            initial_boundary_kind: initial_boundary.kind,
                            final_boundary_kind: final_boundary.kind,
                            step: *step,
                            frame_var: frame_var.ident,
                        },
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::ForExt {
                frame_var,
                final_boundary,
                step,
                ..
            } => {
                self.compile_boundary(final_boundary, &loop_stmt);
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.def_forext_frame();
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: VMLoopKind::ForExt {
                            final_boundary_kind: final_boundary.kind,
                            step: *step,
                            frame_var: frame_var.ident,
                            frame_varidx: self.varidx(frame_var.ident),
                        },
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::While { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionKind::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::DoWhile { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_proc_stmts(body_stmts);
                block_sheet_builder.compile_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionKind::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionKind::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
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

    fn compile_branch_groups(&self, branches: &[Arc<ProcBranch>]) -> Avec<VMBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| match branch.kind {
                    ProcBranchKind::If { ref condition } => Arc::new(VMBranch {
                        opt_condition_sheet: {
                            let mut condition_sheet = self.subsheet_builder();
                            condition_sheet.compile_expr(condition);
                            Some(condition_sheet.finalize())
                        },
                        body: {
                            let mut body_sheet = self.subsheet_builder();
                            body_sheet.compile_proc_stmts(&branch.stmts);
                            body_sheet.finalize()
                        },
                    }),
                    ProcBranchKind::Elif { ref condition } => todo!(),
                    ProcBranchKind::Else => todo!(),
                    ProcBranchKind::Case { ref pattern } => todo!(),
                    ProcBranchKind::Default => todo!(),
                })
                .collect(),
        )
    }
}
