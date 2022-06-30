use super::*;
use semantics_eager::{
    FuncConditionFlowBranch, FuncConditionFlowBranchVariant, ProcConditionBranchVariant,
    ProcConditionFlowBranch, ProcStmt,
};
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_proc_condition_flow(&mut self, branches: &[Arc<ProcConditionFlowBranch>]) {
        for branch in branches {
            match branch.variant {
                ProcConditionBranchVariant::If { ref condition } => {
                    self.write("if ");
                    self.gen_condition(condition);
                }
                ProcConditionBranchVariant::Elif { ref condition } => {
                    self.write(" else if ");
                    self.gen_condition(condition);
                }
                ProcConditionBranchVariant::Else => {
                    self.write(" else");
                }
            }
            self.write(" {");
            self.newline();
            self.gen_proc_stmts(&branch.stmts);
            self.write("}");
        }
    }

    pub(super) fn gen_func_condition_flow(&mut self, branches: &[Arc<FuncConditionFlowBranch>]) {
        for branch in branches {
            match branch.variant {
                FuncConditionFlowBranchVariant::If { ref condition } => {
                    self.write("if ");
                    self.gen_condition(condition);
                }
                FuncConditionFlowBranchVariant::Elif { ref condition } => {
                    self.write(" else if ");
                    self.gen_condition(condition);
                }
                FuncConditionFlowBranchVariant::Else => {
                    self.write(" else");
                }
            }
            self.write(" {");
            self.newline();
            self.gen_func_stmts(&branch.stmts);
            self.write("}");
        }
    }
}
