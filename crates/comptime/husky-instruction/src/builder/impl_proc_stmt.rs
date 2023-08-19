use crate::*;
use avec::Avec;
use husky_primitive_literal_semantics::convert_primitive_literal_to_register;
use husky_vm::{
    Instruction, InstructionData, VMConditionBranch, VMLoopKind, VMPattern, VMPatternBranch,
    __RegistrableSafe,
};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_proc_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_proc_stmt(stmt.clone()));
    }

    fn compile_proc_stmt(&mut self, stmt: HirEagerStmtIdx) {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                ..
            } => {
                self.compile_eager_expr(
                    initial_value,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.def_variable(varname.ident)
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.compile_eager_expr(
                    condition,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.push_instruction(Instruction::new(InstructionData::Assert, stmt))
            }
            ProcStmtVariant::Return { ref result, .. } => {
                self.compile_eager_expr(result, self.sheet.variable_stack.next_stack_idx(), false);
                self.push_instruction(Instruction::new(
                    InstructionData::Return {
                        return_ty: result.intrinsic_ty(),
                    },
                    stmt,
                ));
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.compile_eager_expr(expr, self.sheet.variable_stack.next_stack_idx(), true);
            }
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => self.compile_loop(loop_variant, stmt.clone(), stmts),
            ProcStmtVariant::Break => {
                self.push_instruction(Instruction::new(InstructionData::Break, stmt))
            }
            ProcStmtVariant::ConditionFlow { ref branches, .. } => {
                self.push_instruction(Instruction::new(
                    InstructionData::ConditionFlow {
                        branches: self.compile_proc_condition_flow(branches),
                    },
                    stmt,
                ))
            }
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => {
                self.compile_eager_expr(
                    match_expr,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.push_instruction(Instruction::new(
                    InstructionData::PatternMatch {
                        branches: self.compile_proc_pattern_match(branches),
                    },
                    stmt,
                ))
            }
        }
    }

    fn compile_loop(
        &mut self,
        loop_kind: &LoopVariant,
        loop_stmt: HirEagerStmtIdx,
        body_stmts: &[HirEagerStmtIdx],
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
                block_sheet_builder.def_variable(frame_var.ident);
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionData::Loop {
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
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionData::Loop {
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
                block_sheet_builder.compile_eager_expr(
                    condition,
                    block_sheet_builder.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionData::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionData::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::DoWhile { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_proc_stmts(body_stmts);
                block_sheet_builder.compile_eager_expr(
                    condition,
                    block_sheet_builder.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionData::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionData::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
                    },
                    loop_stmt,
                ));
            }
        }
    }

    fn compile_boundary(&mut self, boundary: &Boundary, loop_stmt: &HirEagerStmtIdx) {
        todo!()
        // if let Some(ref bound) = boundary.opt_bound {
        //     self.compile_eager_expr(bound, self.sheet.variable_stack.next_stack_idx(), false)
        // } else {
        //     self.push_instruction(Instruction::new(
        //         InstructionData::PushLiteralValue {
        //             value: 0i32.to_register(),
        //             explicit: false,
        //             ty: RootBuiltinIdent::I32.into(),
        //         },
        //         loop_stmt.clone(),
        //     ))
        // }
    }

    fn compile_proc_condition_flow(
        &self,
        branches: &[Arc<ProcConditionFlowBranch>],
    ) -> VMConditionBranchs {
        Arc::new(
            branches
                .iter()
                .map(|branch| match branch.variant {
                    ProcConditionFlowBranchVariant::If { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet_builder = self.subsheet_builder();
                                condition_sheet_builder.compile_eager_expr(
                                    condition,
                                    condition_sheet_builder
                                        .sheet
                                        .variable_stack
                                        .next_stack_idx(),
                                    false,
                                );
                                Some(condition_sheet_builder.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    ProcConditionFlowBranchVariant::Elif { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet_builder = self.subsheet_builder();
                                condition_sheet_builder.compile_eager_expr(
                                    condition,
                                    condition_sheet_builder
                                        .sheet
                                        .variable_stack
                                        .next_stack_idx(),
                                    false,
                                );
                                Some(condition_sheet_builder.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    ProcConditionFlowBranchVariant::Else => Arc::new(VMConditionBranch {
                        opt_condition_sheet: None,
                        body: {
                            let mut body_sheet = self.subsheet_builder();
                            body_sheet.compile_proc_stmts(&branch.stmts);
                            body_sheet.finalize()
                        },
                    }),
                })
                .collect(),
        )
    }

    fn compile_proc_pattern_match(
        &self,
        branches: &[Arc<ProcStmtPatternBranch>],
    ) -> Avec<VMPatternBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| {
                    Arc::new(match branch.variant {
                        ProcStmtPatternBranchVariant::Case { ref pattern } => VMPatternBranch {
                            opt_pattern: Some(self.gen_proc_case_pattern(pattern)),
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                        ProcStmtPatternBranchVariant::Default => VMPatternBranch {
                            opt_pattern: None,
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                    })
                })
                .collect(),
        )
    }

    fn gen_proc_case_pattern(&self, pattern: &ProcStmtPattern) -> VMPattern {
        todo!()
        // match pattern.variant {
        //     ProcStmtPatternVariant::PrimitiveLiteral(data) => {
        //         VMPattern::Primitive(convert_primitive_literal_to_register(data, pattern.ty))
        //     }
        //     ProcStmtPatternVariant::OneOf { ref subpatterns } => VMPattern::Or(
        //         subpatterns
        //             .iter()
        //             .map(|subpattern| self.gen_proc_case_pattern(subpattern))
        //             .collect(),
        //     ),
        //     ProcStmtPatternVariant::EnumLiteral(route) => VMPattern::EnumKind {
        //         kind_idx: self.db.enum_literal_to_i32(route),
        //     },
        // }
    }
}
