mod pattern;
mod stmt;

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
            HirEagerExpr::SelfType => builder.self_ty(),
            HirEagerExpr::SelfValue => builder.self_value(),
            HirEagerExpr::Binary { lopd, opr, ropd } => {
                lopd.transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                ropd.transpile_to_rust(builder)
            }
            HirEagerExpr::Be { src, target } => builder.macro_name(RustMacroName::Matches),
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
            } => {
                owner_hir_expr_idx.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder)
            }
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
            } =>
            /* ad hoc */
            {
                ()
            }
            // todo!(),
            HirEagerExpr::Todo => todo!(),
            HirEagerExpr::AssociatedFn {
                associated_item_path,
            } => associated_item_path.transpile_to_rust(builder),
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
