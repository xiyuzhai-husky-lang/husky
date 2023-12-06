pub(crate) mod on_site;
pub(crate) mod pattern;
pub(crate) mod precedence;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::precedence::hir_eager_expr_precedence;
use crate::*;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_hir_eager_expr::{
    HirEagerCondition, HirEagerElifBranch, HirEagerElseBranch, HirEagerExprData, HirEagerExprIdx,
    HirEagerExprRegion, HirEagerIfBranch, HirEagerLetVariablesPattern, HirEagerPatternExpr,
    HirEagerPatternExprIdx, HirEagerRitchieParameterArgumentMatch, HirEagerStmtData,
    HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::binary::HirBinaryOpr;
use husky_opr::BinaryClosedOpr;

impl TranspileToRust<HirEagerExprRegion> for (RustPrecedenceRange, HirEagerExprIdx) {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (precedence_range, slf) = self;
        let data = slf.data(builder.hir_eager_expr_arena());
        let precedence = hir_eager_expr_precedence(data);
        if !precedence_range.include(precedence) {
            builder.bracketed_list_with(RustBracket::Par, |builder| {
                transpile_hir_eager_expr_to_rust(data, precedence, builder)
            })
        } else {
            transpile_hir_eager_expr_to_rust(data, precedence, builder)
        }
    }
}

fn transpile_hir_eager_expr_to_rust(
    data: &HirEagerExprData,
    precedence: RustPrecedence,
    builder: &mut RustTranspilationBuilder<HirEagerExprRegion>,
) {
    let geq = |opd| (RustPrecedenceRange::Geq(precedence), opd);
    let greater = |opd| (RustPrecedenceRange::Greater(precedence), opd);
    let db = builder.db();
    match *data {
        HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
        HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
            principal_entity_path.transpile_to_rust(builder);
            match principal_entity_path {
                PrincipalEntityPath::Module(_) => unreachable!(),
                PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)) => {
                    match path.fugitive_kind(db) {
                        FugitiveKind::FunctionFn => (),
                        FugitiveKind::Val => builder.bracketed(RustBracket::Par, |_| ()),
                        FugitiveKind::FunctionGn => unreachable!(),
                        FugitiveKind::AliasType => unreachable!(),
                    }
                }
                PrincipalEntityPath::TypeVariant(_) => (),
                PrincipalEntityPath::MajorItem(_) => (),
            }
        }
        HirEagerExprData::ConstSymbol(_) => todo!(),
        HirEagerExprData::Variable(hir_eager_runtime_symbol_idx) => {
            hir_eager_runtime_symbol_idx.transpile_to_rust(builder)
        }
        HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
            HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                (RustPrecedenceRange::Geq(RustPrecedence::Suffix), lopd).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.rem_eulid();
                builder.bracketed_list_with(RustBracket::Par, |builder| {
                    any_precedence(ropd).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                (RustPrecedenceRange::Geq(RustPrecedence::Suffix), lopd).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.pow();
                builder.bracketed_list_with(RustBracket::Par, |builder| {
                    any_precedence(ropd).transpile_to_rust(builder)
                })
            }
            _ => {
                geq(lopd).transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                greater(ropd).transpile_to_rust(builder)
            }
        },
        HirEagerExprData::Be { src: _, target: _ } => builder.macro_name(RustMacroName::Matches),
        HirEagerExprData::Prefix {
            opr,
            opd_hir_expr_idx,
        } => {
            // todo: check some details
            opr.transpile_to_rust(builder);
            geq(opd_hir_expr_idx).transpile_to_rust(builder)
        }
        HirEagerExprData::Suffix {
            opd_hir_expr_idx,
            opr,
        } => {
            geq(opd_hir_expr_idx).transpile_to_rust(builder);
            opr.transpile_to_rust(builder)
        }
        HirEagerExprData::TypeConstructorFnCall {
            function_hir_eager_expr_idx,
            path,
            ref instantiation,
            ref item_groups,
        } => {
            builder.ty_constructor(path);
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::TypeVariantConstructorCall {
            function_hir_eager_expr_idx,
            path,
            ref instantiation,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::FunctionFnCall {
            function_hir_eager_expr_idx,
            path,
            ref instantiation,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::AssociatedFunctionFnCall {
            function_hir_eager_expr_idx,
            path,
            ref instantiation,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::PropsStructField {
            owner_hir_expr_idx,
            ident,
        } => {
            geq(owner_hir_expr_idx).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder)
        }
        HirEagerExprData::MemoizedField {
            owner_hir_expr_idx,
            ident,
            ..
        } => {
            geq(owner_hir_expr_idx).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder);
            builder.bracketed(RustBracket::Par, |_| ())
        }
        HirEagerExprData::MethodFnCall {
            self_argument,
            ident,
            path,
            ref instantiation,
            ref item_groups,
        } => {
            geq(self_argument).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::NewTuple { items: _ } => {
            todo!()
        }
        HirEagerExprData::Index {
            owner_hir_expr_idx,
            ref items,
        } => {
            geq(owner_hir_expr_idx).transpile_to_rust(builder);
            builder
                .bracketed_comma_list(RustBracket::Box, items.iter().copied().map(any_precedence))
        }
        HirEagerExprData::NewList { ref items } => {
            builder.macro_name(RustMacroName::Vec);
            builder
                .bracketed_comma_list(RustBracket::Box, items.iter().copied().map(any_precedence))
        }
        HirEagerExprData::Block { stmts } => stmts.transpile_to_rust(builder),
        HirEagerExprData::EmptyHtmlTag {
            function_ident: _,
            arguments: _,
        } =>
        /* ad hoc */
        {
            ()
        }
        HirEagerExprData::Todo => {
            builder.macro_name(RustMacroName::Todo);
            builder.bracketed(RustBracket::Par, |_| ())
        }
        HirEagerExprData::Unreachable => {
            builder.macro_name(RustMacroName::Unreachable);
            builder.bracketed(RustBracket::Par, |_| ())
        }
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => associated_item_path.transpile_to_rust(builder),
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerRitchieParameterArgumentMatch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            &HirEagerRitchieParameterArgumentMatch::Regular(
                param,
                hir_eager_expr_idx,
                coersion,
            ) => any_precedence(hir_eager_expr_idx).transpile_to_rust(builder),
            HirEagerRitchieParameterArgumentMatch::Variadic => todo!(),
            HirEagerRitchieParameterArgumentMatch::Keyed => todo!(),
        }
    }
}
