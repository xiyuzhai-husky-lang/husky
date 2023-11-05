use crate::*;
use husky_hir_eager_expr::{
    HirEagerCallListItemGroup, HirEagerCondition, HirEagerElifBranch, HirEagerElseBranch,
    HirEagerExpr, HirEagerExprIdx, HirEagerIfBranch, HirEagerLetVariablesPattern,
    HirEagerPatternExpr, HirEagerPatternExprIdx, HirEagerStmt, HirEagerStmtIdx,
    HirEagerStmtIdxRange,
};
use husky_term_prelude::TermLiteral;

impl TranspileToRust for HirEagerExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_expr_arena()) {
            HirEagerExpr::Literal(term_literal) => term_literal.transpile_to_rust(builder),
            HirEagerExpr::PrincipalEntityPath(principal_entity_path) => {
                principal_entity_path.transpile_to_rust(builder)
            }
            HirEagerExpr::InheritedSymbol { ident } => ident.transpile_to_rust(builder),
            HirEagerExpr::CurrentSymbol { ident } => ident.transpile_to_rust(builder),
            HirEagerExpr::FrameVarDecl { ident } => ident.transpile_to_rust(builder),
            HirEagerExpr::SelfType => todo!(),
            HirEagerExpr::SelfValue => todo!(),
            HirEagerExpr::Binary { lopd, opr, ropd } => {
                lopd.transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                ropd.transpile_to_rust(builder)
            }
            HirEagerExpr::Be { src, target } => todo!(),
            HirEagerExpr::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                opr.transpile_to_rust(builder);
                opd_hir_expr_idx.transpile_to_rust(builder)
            }
            HirEagerExpr::Suffix {
                opd_hir_expr_idx,
                opr,
            } => {
                opd_hir_expr_idx.transpile_to_rust(builder);
                opr.transpile_to_rust(builder)
            }
            HirEagerExpr::FnCall {
                function_hir_expr_idx,
                template_arguments,
                item_groups,
            } => {
                function_hir_expr_idx.transpile_to_rust(builder);
                if let Some(template_arguments) = template_arguments {
                    todo!()
                }
                builder.bracketed_comma_list(RustBracket::Par, item_groups)
            }
            HirEagerExpr::Field {
                owner_hir_expr_idx,
                ident,
            } => todo!(),
            HirEagerExpr::MethodCall {
                self_argument,
                ident,
                template_arguments,
                item_groups,
            } => {
                self_argument.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder);
                if let Some(template_arguments) = template_arguments {
                    todo!()
                }
                builder.bracketed_comma_list(RustBracket::Par, item_groups)
            }
            HirEagerExpr::NewTuple { items } => {
                todo!()
            }
            HirEagerExpr::Index {
                owner_hir_expr_idx,
                items,
            } => {
                owner_hir_expr_idx.transpile_to_rust(builder);
                builder.bracketed_comma_list(RustBracket::Box, items)
            }
            HirEagerExpr::NewList { items } => {
                builder.macro_name(RustMacroName::Vec);
                builder.bracketed_comma_list(RustBracket::Box, items)
            }
            HirEagerExpr::Block { stmts } => {
                for stmt in stmts {
                    stmt.transpile_to_rust(builder)
                }
            }
            HirEagerExpr::EmptyHtmlTag {
                function_ident,
                arguments,
            } => todo!(),
            HirEagerExpr::Todo => todo!(),
            HirEagerExpr::AssociatedFn => todo!(),
            HirEagerExpr::AssociatedGn => todo!(),
        }
    }
}

impl TranspileToRust for HirEagerCallListItemGroup {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            HirEagerCallListItemGroup::Regular(hir_eager_expr_idx) => {
                hir_eager_expr_idx.transpile_to_rust(builder)
            }
            HirEagerCallListItemGroup::Variadic => todo!(),
            HirEagerCallListItemGroup::Keyed => todo!(),
        }
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

impl TranspileToRust for HirEagerPatternExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_pattern_expr_arena()) {
            HirEagerPatternExpr::Literal(_) => todo!(),
            HirEagerPatternExpr::Ident { ident } => ident.transpile_to_rust(builder),
            HirEagerPatternExpr::Unit(_) => todo!(),
            HirEagerPatternExpr::Tuple { path, fields } => todo!(),
            HirEagerPatternExpr::Props { path, fields } => todo!(),
            HirEagerPatternExpr::OneOf { options } => todo!(),
            HirEagerPatternExpr::Binding { ident, src } => todo!(),
            HirEagerPatternExpr::Range { start, end } => todo!(),
        }
    }
}

impl TranspileToRust for HirEagerStmtIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_stmt_arena()) {
            HirEagerStmt::Let {
                pattern,
                initial_value,
            } => builder.on_new_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Let);
                pattern.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Eq);
                initial_value.transpile_to_rust(builder)
            }),
            HirEagerStmt::Return { result } => todo!(),
            HirEagerStmt::Require { condition } => todo!(),
            HirEagerStmt::Assert { condition } => todo!(),
            HirEagerStmt::Break => {
                builder.on_new_semicolon_line(|builder| builder.keyword(RustKeyword::Break))
            }
            HirEagerStmt::Eval { expr_idx } => builder.on_new_semicolon_line(|builder| {
                expr_idx.transpile_to_rust(builder);
            }),
            HirEagerStmt::ForBetween { particulars, block } => todo!(),
            HirEagerStmt::ForExt { particulars, block } => todo!(),
            HirEagerStmt::ForIn { condition, block } => todo!(),
            HirEagerStmt::While { condition, stmts } => {
                builder.keyword(RustKeyword::While);
                condition.transpile_to_rust(builder);
                stmts.transpile_to_rust(builder)
            }
            HirEagerStmt::DoWhile { condition, block } => todo!(),
            HirEagerStmt::IfElse {
                if_branch,
                elif_branches,
                else_branch,
            } => builder.on_new_line(|builder| {
                if_branch.transpile_to_rust(builder);
                for elif_branch in elif_branches {
                    elif_branch.transpile_to_rust(builder)
                }
                else_branch.transpile_to_rust(builder)
            }),
            HirEagerStmt::Match {} => todo!(),
        }
    }
}

impl TranspileToRust for HirEagerCondition {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        self.hir_eager_expr_idx().transpile_to_rust(builder)
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
