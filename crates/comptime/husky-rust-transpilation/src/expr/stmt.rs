use super::*;

impl TranspileToRust for HirEagerStmtIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match *self.data(builder.hir_eager_stmt_arena()) {
            HirEagerStmt::Let {
                pattern,
                initial_value,
            } => builder.on_new_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Let);
                pattern.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Eq);
                any_precedence(initial_value).transpile_to_rust(builder)
            }),
            HirEagerStmt::Return { result } => builder.on_new_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Return);
                any_precedence(result).transpile_to_rust(builder)
            }),
            HirEagerStmt::Require { condition } => builder.on_new_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Require);
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                    condition.transpile_to_rust(builder)
                })
            }),
            HirEagerStmt::Assert { condition } => builder.on_new_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Assert);
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                    condition.transpile_to_rust(builder)
                })
            }),
            HirEagerStmt::Break => {
                builder.on_new_semicolon_line(|builder| builder.keyword(RustKeyword::Break))
            }
            HirEagerStmt::Eval { expr_idx } => builder.on_new_semicolon_line(|builder| {
                any_precedence(expr_idx).transpile_to_rust(builder);
            }),
            HirEagerStmt::ForBetween {
                ref particulars,
                block,
            } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::For);
                block.transpile_to_rust(builder)
            }),
            HirEagerStmt::ForExt { particulars, block } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::For);
                block.transpile_to_rust(builder)
            }),
            HirEagerStmt::ForIn { condition, block } => todo!(),
            HirEagerStmt::While { condition, stmts } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::While);
                condition.transpile_to_rust(builder);
                stmts.transpile_to_rust(builder)
            }),
            HirEagerStmt::DoWhile { condition, block } => {
                builder.on_new_line(|builder| {
                    builder.keyword(RustKeyword::While);
                    true.transpile_to_rust(builder);
                })
                // block.transpile_to_rust(builder)
            }
            HirEagerStmt::IfElse {
                if_branch,
                ref elif_branches,
                else_branch,
            } => builder.on_new_line(|builder| {
                if_branch.transpile_to_rust(builder);
                for elif_branch in elif_branches {
                    elif_branch.transpile_to_rust(builder)
                }
                else_branch.transpile_to_rust(builder)
            }),
            HirEagerStmt::Match {} => {
                builder.on_new_line(|builder| builder.keyword(RustKeyword::Match))
            }
        }
    }
}

impl TranspileToRust for HirEagerCondition {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        any_precedence(self.hir_eager_expr_idx()).transpile_to_rust(builder)
    }
}

impl TranspileToRust for HirEagerLetVariablesPattern {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        self.pattern_expr_idx().transpile_to_rust(builder);
        if let Some(ty) = self.ty() {
            builder.punctuation(RustPunctuation::Colon);
            ty.transpile_to_rust(builder)
        }
    }
}

impl TranspileToRust for HirEagerIfBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust for HirEagerElifBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::Else);
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust for HirEagerElseBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::Else);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust for HirEagerStmtIdxRange {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.curly_block(|builder| {
            for stmt in self {
                stmt.transpile_to_rust(builder)
            }
        })
    }
}
