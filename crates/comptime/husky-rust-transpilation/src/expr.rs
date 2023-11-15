mod pattern;
pub(crate) mod precedence;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::precedence::hir_eager_expr_precedence;
use crate::*;
use husky_hir_eager_expr::{
    HirEagerCallListItemGroup, HirEagerCondition, HirEagerElifBranch, HirEagerElseBranch,
    HirEagerExprData, HirEagerExprIdx, HirEagerIfBranch, HirEagerLetVariablesPattern,
    HirEagerPatternExpr, HirEagerPatternExprIdx, HirEagerStmt, HirEagerStmtIdx,
    HirEagerStmtIdxRange,
};
use husky_hir_opr::binary::HirBinaryOpr;
use husky_opr::BinaryClosedOpr;

impl TranspileToRust for (RustPrecedenceRange, HirEagerExprIdx) {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let (precedence_range, slf) = self;
        let data = slf.data(builder.hir_eager_expr_arena());
        let precedence = hir_eager_expr_precedence(data);
        if !precedence_range.include(precedence) {
            builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
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
    builder: &mut RustTranspilationBuilder,
) {
    let geq = |opd| (RustPrecedenceRange::Geq(precedence), opd);
    let greater = |opd| (RustPrecedenceRange::Greater(precedence), opd);
    match *data {
        HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
        HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
            principal_entity_path.transpile_to_rust(builder)
        }
        HirEagerExprData::ConstSymbol(_) => todo!(),
        HirEagerExprData::Variable(hir_eager_variable_idx) => {
            hir_eager_variable_idx.transpile_to_rust(builder)
        }
        HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
            HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                (RustPrecedenceRange::Geq(RustPrecedence::Suffix), lopd).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.rem_eulid();
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                    any_precedence(ropd).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                (RustPrecedenceRange::Geq(RustPrecedence::Suffix), lopd).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.pow();
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
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
        HirEagerExprData::TypeConstructorCall {
            function_hir_eager_expr_idx,
            path,
            ref template_arguments,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            if let Some(_template_arguments) = template_arguments {
                todo!()
            }
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::TypeVariantConstructorCall {
            function_hir_eager_expr_idx,
            path,
            ref template_arguments,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            if let Some(_template_arguments) = template_arguments {
                todo!()
            }
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::FunctionFnCall {
            function_hir_eager_expr_idx,
            path,
            ref template_arguments,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            if let Some(_template_arguments) = template_arguments {
                todo!()
            }
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::AssociatedItemFunctionFnCall {
            function_hir_eager_expr_idx,
            path,
            ref parent_template_arguments,
            ref template_arguments,
            ref item_groups,
        } => {
            geq(function_hir_eager_expr_idx).transpile_to_rust(builder);
            if let Some(_template_arguments) = template_arguments {
                todo!()
            }
            builder.bracketed_comma_list(RustBracket::Par, item_groups)
        }
        HirEagerExprData::Field {
            owner_hir_expr_idx,
            ident,
        } => {
            geq(owner_hir_expr_idx).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder)
        }
        HirEagerExprData::MethodFnCall {
            self_argument,
            ident,
            path,
            ref template_arguments,
            ref item_groups,
        } => {
            geq(self_argument).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder);
            if let Some(_template_arguments) = template_arguments {
                todo!()
            }
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
        // todo!(),
        HirEagerExprData::Todo => todo!(),
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => associated_item_path.transpile_to_rust(builder),
    }
}

impl TranspileToRust for HirEagerCallListItemGroup {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            &HirEagerCallListItemGroup::Regular(hir_eager_expr_idx) => {
                any_precedence(hir_eager_expr_idx).transpile_to_rust(builder)
            }
            HirEagerCallListItemGroup::Variadic => todo!(),
            HirEagerCallListItemGroup::Keyed => todo!(),
        }
    }
}
