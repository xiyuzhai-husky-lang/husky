use crate::*;
use husky_hir_eager_expr::{HirEagerExpr, HirEagerExprIdx};

impl TranspileToRust for HirEagerExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_expr_arena()) {
            HirEagerExpr::Literal(_) => todo!(),
            HirEagerExpr::PrincipalEntityPath(_) => todo!(),
            HirEagerExpr::InheritedSymbol { ident } => todo!(),
            HirEagerExpr::CurrentSymbol { ident } => todo!(),
            HirEagerExpr::FrameVarDecl { ident } => todo!(),
            HirEagerExpr::SelfType => todo!(),
            HirEagerExpr::SelfValue => todo!(),
            HirEagerExpr::Binary { lopd, opr, ropd } => todo!(),
            HirEagerExpr::Be { src, target } => todo!(),
            HirEagerExpr::Prefix {
                opr,
                opd_hir_expr_idx,
            } => todo!(),
            HirEagerExpr::Suffix {
                opd_hir_expr_idx,
                opr,
            } => todo!(),
            HirEagerExpr::FnCall {
                function_hir_expr_idx,
                template_arguments,
                item_groups,
            } => todo!(),
            HirEagerExpr::Field {
                owner_hir_expr_idx,
                ident,
            } => todo!(),
            HirEagerExpr::MethodCall {
                self_argument,
                ident,
                template_arguments,
                item_groups,
            } => todo!(),
            HirEagerExpr::NewTuple { items } => todo!(),
            HirEagerExpr::Index {
                owner_hir_expr_idx,
                items,
            } => todo!(),
            HirEagerExpr::NewList { items } => todo!(),
            HirEagerExpr::Block { stmts } => todo!(),
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
