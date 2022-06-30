use super::*;
use ast::{CasePattern, CasePatternVariant};
use semantics_eager::{
    FuncPatternBranch, FuncPatternBranchVariant, ProcPatternBranch, ProcPatternBranchVariant,
};
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub fn gen_func_match_pattern(
        &mut self,
        ref match_expr: &EagerExpr,
        ref branches: &[Arc<FuncPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(match_expr);
        self.write(" {");
        self.newline();
        for branch in branches.iter() {
            self.indent();
            match branch.variant {
                FuncPatternBranchVariant::Case { ref pattern } => {
                    self.gen_case_pattern(pattern);
                }
                FuncPatternBranchVariant::Default => self.write("_"),
            }
            self.write(" => {");
            self.newline();
            self.gen_func_stmts(&branch.stmts);
            self.write(" => }");
        }
        self.write("}");
    }

    pub fn gen_proc_match_pattern(
        &mut self,
        ref match_expr: &EagerExpr,
        ref branches: &[Arc<ProcPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(match_expr);
        self.write(" {");
        self.newline();
        for branch in branches.iter() {
            self.indent();
            match branch.variant {
                ProcPatternBranchVariant::Case { ref pattern } => {
                    self.gen_case_pattern(pattern);
                }
                ProcPatternBranchVariant::Default => self.write("_"),
            }
            self.write(" => {");
            self.newline();
            self.gen_proc_stmts(&branch.stmts);
            self.write(" => }");
        }
        self.write("}");
    }

    fn gen_case_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(v) => {
                let v: String = v.into();
                self.write(&(v))
            }
            CasePatternVariant::OneOf { ref patterns } => {
                for (i, pattern) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.gen_case_pattern(pattern)
                }
            }
            CasePatternVariant::EnumLiteral(value) => self.write(&format!("{value}")),
        }
    }
}
