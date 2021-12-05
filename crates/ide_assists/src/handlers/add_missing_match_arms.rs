use std::iter::{self, Peekable};

use either::Either;
use hir::{Adt, HasSource, ModuleDef, Semantics};
use ide_db::helpers::{mod_path_to_ast, FamousDefs};
use ide_db::RootDatabase;
use itertools::Itertools;
use syntax::ast::{self, MatchArm, MatchArmList, MatchExpr, Pat};

use crate::{
    utils::{self, render_snippet, Cursor},
    AssistContext, AssistId, AssistKind, Assists,
};
pub(crate) fn add_missing_match_arms(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn cursor_at_trivial_match_arm_list(
    ctx: &AssistContext,
    match_expr: &MatchExpr,
    match_arm_list: &MatchArmList,
) -> Option<()> {
    todo!()
}

fn is_variant_missing(existing_pats: &[Pat], var: &Pat) -> bool {
    !existing_pats
        .iter()
        .any(|pat| does_pat_match_variant(pat, var))
}

// Fixme: this is still somewhat limited, use hir_ty::diagnostics::match_check?
fn does_pat_match_variant(pat: &Pat, var: &Pat) -> bool {
    todo!()
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum ExtendedEnum {
    Bool,
    Enum(hir::Enum),
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum ExtendedVariant {
    True,
    False,
    Variant(hir::Variant),
}

fn lift_enum(e: hir::Enum) -> ExtendedEnum {
    ExtendedEnum::Enum(e)
}

impl ExtendedEnum {
    fn variants(self, db: &RootDatabase) -> Vec<ExtendedVariant> {
        todo!()
    }
}

fn resolve_enum_def(sema: &Semantics<RootDatabase>, expr: &ast::Expr) -> Option<ExtendedEnum> {
    todo!()
}

fn resolve_tuple_of_enum_def(
    sema: &Semantics<RootDatabase>,
    expr: &ast::Expr,
) -> Option<Vec<ExtendedEnum>> {
    todo!()
}

fn build_pat(db: &RootDatabase, module: hir::Module, var: ExtendedVariant) -> Option<ast::Pat> {
    todo!()
}
