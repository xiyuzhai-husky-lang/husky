use common::*;

use hir::{AsAssocItem, Semantics};
use itertools::Itertools;
use husky_lang_db::{
    helpers::{for_each_tail_expr, node_ext::block_as_lone_tail, FamousDefs},
    HuskyLangDatabase,
};
use syntax::{ast, SyntaxNode};

use crate::{
    utils::{invert_boolean_expression, unwrap_trivial_block},
    AssistContext, AssistId, AssistKind, Assists,
};

pub(crate) fn convert_if_to_bool_then(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

pub(crate) fn convert_bool_then_to_if(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn option_variants(
    sema: &Semantics<HuskyLangDatabase>,
    expr: &SyntaxNode,
) -> Option<(hir::Variant, hir::Variant)> {
    todo!()
}

/// Traverses the expression checking if it contains `return` or `?` expressions or if any tail is not a `Some(expr)` expression.
/// If any of these conditions are met it is impossible to rewrite this as a `bool::then` call.
fn is_invalid_body(
    sema: &Semantics<HuskyLangDatabase>,
    some_variant: hir::Variant,
    expr: &ast::Expr,
) -> bool {
    todo!()
}

fn block_is_none_variant(
    sema: &Semantics<HuskyLangDatabase>,
    block: &ast::BlockExpr,
    none_variant: hir::Variant,
) -> bool {
    todo!()
}
