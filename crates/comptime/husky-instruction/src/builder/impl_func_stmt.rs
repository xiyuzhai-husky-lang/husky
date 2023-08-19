use crate::*;
use avec::Avec;
use husky_primitive_literal_semantics::convert_primitive_literal_to_register;
use husky_vm::{Instruction, InstructionData, VMConditionBranch, VMPattern, VMPatternBranch};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_func_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_func_stmt(stmt.clone()));
    }

    fn compile_func_stmt(&mut self, stmt: HirEagerStmtIdx) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.compile_eager_expr(
                    initial_value,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.def_variable(varname.ident)
            }
            FuncStmtVariant::Assert { ref condition } => {
                self.compile_eager_expr(
                    condition,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.push_instruction(Instruction::new(InstructionData::Assert, stmt))
            }
            FuncStmtVariant::Require { ref condition, .. } => {
                self.compile_eager_expr(
                    condition,
                    self.sheet.variable_stack.next_stack_idx(),
                    false,
                );
                self.push_instruction(Instruction::new(InstructionData::Require, stmt))
            }
            FuncStmtVariant::Return { ref result, .. } => {
                self.compile_eager_expr(result, self.sheet.variable_stack.next_stack_idx(), false);
                self.push_instruction(Instruction::new(
                    InstructionData::Return {
                        return_ty: result.intrinsic_ty(),
                    },
                    stmt,
                ));
            }
            FuncStmtVariant::ConditionFlow { ref branches } => {
                self.push_instruction(Instruction::new(
                    InstructionData::ConditionFlow {
                        branches: self.compile_func_condition_flow(branches),
                    },
                    stmt,
                ))
            }
            FuncStmtVariant::Match {
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
                        branches: self.compile_func_pattern_match(branches),
                    },
                    stmt,
                ))
            }
        }
    }

    fn compile_func_condition_flow(
        &self,
        branches: &[Arc<FuncConditionFlowBranch>],
    ) -> VMConditionBranchs {
        Arc::new(
            branches
                .iter()
                .map(|branch| match branch.variant {
                    FuncConditionFlowBranchVariant::If { ref condition } => {
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
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    FuncConditionFlowBranchVariant::Elif { ref condition } => {
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
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    FuncConditionFlowBranchVariant::Else => Arc::new(VMConditionBranch {
                        opt_condition_sheet: None,
                        body: {
                            let mut body_sheet = self.subsheet_builder();
                            body_sheet.compile_func_stmts(&branch.stmts);
                            body_sheet.finalize()
                        },
                    }),
                })
                .collect(),
        )
    }

    fn compile_func_pattern_match(
        &self,
        branches: &[Arc<FuncStmtPatternBranch>],
    ) -> Avec<VMPatternBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| {
                    Arc::new(match branch.variant {
                        FuncStmtPatternBranchVariant::Case { ref pattern } => VMPatternBranch {
                            opt_pattern: Some(self.gen_func_case_pattern(pattern)),
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                        FuncStmtPatternBranchVariant::Default => VMPatternBranch {
                            opt_pattern: None,
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                    })
                })
                .collect(),
        )
    }

    fn gen_func_case_pattern(&self, pattern: &FuncStmtPattern) -> VMPattern {
        match pattern.variant {
            FuncStmtPatternVariant::PrimitiveLiteral(data) => {
                VMPattern::Primitive(convert_primitive_literal_to_register(data, pattern.ty))
            }
            FuncStmtPatternVariant::OneOf { ref subpatterns } => VMPattern::Or(
                subpatterns
                    .iter()
                    .map(|subpattern| self.gen_func_case_pattern(subpattern))
                    .collect(),
            ),
            FuncStmtPatternVariant::EnumLiteral(route) => VMPattern::EnumKind {
                kind_idx: self.db.enum_literal_to_i32(route),
            },
        }
    }
}
