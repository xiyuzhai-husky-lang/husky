use super::*;
use husky_eager_semantics::{
    FuncStmtPattern, FuncStmtPatternBranch, FuncStmtPatternBranchVariant, FuncStmtPatternVariant,
    ProcStmtPattern, ProcStmtPatternBranch, ProcStmtPatternBranchVariant, ProcStmtPatternVariant,
};
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub fn gen_func_match_pattern(
        &mut self,
        ref match_expr: HirEagerExprIdx,
        indent: u8,
        ref branches: &[Arc<FuncStmtPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(indent, match_expr);
        self.write(" {");
        self.newline();
        let mut has_default = false;
        for branch in branches.iter() {
            self.indent(indent + 4);
            match branch.variant {
                FuncStmtPatternBranchVariant::Case { ref pattern } => {
                    self.gen_func_case_pattern(pattern);
                }
                FuncStmtPatternBranchVariant::Default => {
                    has_default = true;
                    self.write("_")
                }
            }
            self.write(" => {");
            self.newline();
            self.gen_func_stmts(&branch.stmts);
            self.indent(indent + 4);
            self.write("}\n");
        }
        if !has_default {
            self.indent(indent + 4);
            self.write("_ => panic!(),\n")
        }
        self.indent(indent);
        self.write("}");
    }

    pub fn gen_proc_match_pattern(
        &mut self,
        ref match_expr: HirEagerExprIdx,
        indent: u8,
        ref branches: &[Arc<ProcStmtPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(indent, match_expr);
        self.write(" {");
        self.newline();
        let mut has_default = false;
        for branch in branches.iter() {
            self.indent(indent + 4);
            match branch.variant {
                ProcStmtPatternBranchVariant::Case { ref pattern } => {
                    self.gen_proc_case_pattern(pattern);
                }
                ProcStmtPatternBranchVariant::Default => {
                    has_default = true;
                    self.write("_")
                }
            }
            self.write(" => {");
            self.newline();
            self.gen_proc_stmts(&branch.stmts);
            self.indent(indent + 4);
            self.write("}\n");
        }
        if !has_default {
            self.indent(indent);
            self.write("_ => panic!(),\n")
        }
        self.indent(indent);
        self.write("}");
    }

    fn gen_func_case_pattern(&mut self, pattern: &FuncStmtPattern) {
        match pattern.variant {
            FuncStmtPatternVariant::PrimitiveLiteral(v) => {
                let v: String = v.into();
                self.write(&(v))
            }
            FuncStmtPatternVariant::OneOf { ref subpatterns } => {
                for (i, subpattern) in subpatterns.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.gen_func_case_pattern(subpattern)
                }
            }
            FuncStmtPatternVariant::EnumLiteral(item_path) => {
                self.gen_item_route(item_path, EntityRouteRole::Other)
            }
        }
    }

    fn gen_proc_case_pattern(&mut self, pattern: &ProcStmtPattern) {
        match pattern.variant {
            ProcStmtPatternVariant::PrimitiveLiteral(v) => {
                let v: String = v.into();
                self.write(&(v))
            }
            ProcStmtPatternVariant::OneOf { ref subpatterns } => {
                for (i, subpattern) in subpatterns.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.gen_proc_case_pattern(subpattern)
                }
            }
            ProcStmtPatternVariant::EnumLiteral(item_path) => {
                self.gen_item_route(item_path, EntityRouteRole::Other)
            }
        }
    }
}
