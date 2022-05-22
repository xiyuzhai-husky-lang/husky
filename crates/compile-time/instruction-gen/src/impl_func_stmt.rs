use crate::*;
use avec::Avec;
use vm::{EagerContract, InitKind, Instruction, InstructionKind, VMPatternBranch};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_func_stmts(&mut self, stmts: &[Arc<FuncStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_func_stmt(stmt.clone()));
    }

    fn compile_func_stmt(&mut self, stmt: Arc<FuncStmt>) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.compile_eager_expr(initial_value);
                self.def_variable(varname.ident)
            }
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => {
                self.compile_eager_expr(result);
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
            FuncStmtVariant::ConditionFlow { .. } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => {
                self.compile_eager_expr(match_expr);
                self.push_instruction(Instruction::new(
                    InstructionKind::PatternMatch {
                        branches: self.compile_func_pattern_match(branches),
                    },
                    stmt,
                ))
            }
            FuncStmtVariant::ReturnXml { ref xml_expr } => {
                self.compile_xml_expr(xml_expr.clone());
                self.push_instruction(Instruction::new(InstructionKind::Return, stmt));
            }
        }
    }

    fn compile_func_pattern_match(
        &self,
        branches: &[Arc<FuncPatternBranch>],
    ) -> Avec<VMPatternBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| {
                    Arc::new(match branch.variant {
                        FuncPatternBranchVariant::Case { ref pattern } => VMPatternBranch {
                            opt_pattern: Some(pattern.compile()),
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                        FuncPatternBranchVariant::Default => VMPatternBranch {
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
}
