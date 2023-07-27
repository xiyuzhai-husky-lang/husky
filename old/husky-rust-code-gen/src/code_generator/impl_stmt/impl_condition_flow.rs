use super::*;
use fold::Indent;
use husky_eager_semantics::{
    FuncConditionFlowBranch, FuncConditionFlowBranchVariant, ProcConditionFlowBranch,
    ProcConditionFlowBranchVariant,
};
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_proc_condition_flow(
        &mut self,
        indent: Indent,
        branches: &[Arc<ProcConditionFlowBranch>],
    ) {
        for branch in branches {
            match branch.variant {
                ProcConditionFlowBranchVariant::If { ref condition } => {
                    self.write("if ");
                    self.gen_condition(indent + 4, condition);
                }
                ProcConditionFlowBranchVariant::Elif { ref condition } => {
                    self.write(" else if ");
                    self.gen_condition(indent + 4, condition);
                }
                ProcConditionFlowBranchVariant::Else => {
                    self.write(" else");
                }
            }
            self.write(" {");
            self.newline();
            self.gen_proc_stmts(&branch.stmts);
            self.indent(indent);
            self.write("}");
        }
    }

    pub(super) fn gen_func_condition_flow(
        &mut self,
        indent: Indent,
        branches: &[Arc<FuncConditionFlowBranch>],
    ) {
        for branch in branches {
            match branch.variant {
                FuncConditionFlowBranchVariant::If { ref condition } => {
                    self.write("if ");
                    self.gen_condition(indent, condition);
                }
                FuncConditionFlowBranchVariant::Elif { ref condition } => {
                    self.write(" else if ");
                    self.gen_condition(indent, condition);
                }
                FuncConditionFlowBranchVariant::Else => {
                    self.write(" else");
                }
            }
            self.write(" {");
            self.newline();
            self.gen_func_stmts(&branch.stmts);
            self.indent(indent);
            self.write("}");
        }
    }
}
