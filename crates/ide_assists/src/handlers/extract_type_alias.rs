use either::Either;
use ide_db::helpers::node_ext::walk_ty;
use itertools::Itertools;
use syntax::ast::{self, edit::IndentLevel};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: extract_type_alias
//
// Extracts the selected type as a type alias.
//
// ```
// struct S {
//     field: $0(u8, u8, u8)$0,
// }
// ```
// ->
// ```
// type $0Type = (u8, u8, u8);
//
// struct S {
//     field: Type,
// }
// ```
pub(crate) fn extract_type_alias(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn collect_used_generics<'gp>(
    ty: &ast::Type,
    known_generics: &'gp [ast::GenericParam],
) -> Vec<&'gp ast::GenericParam> {
    todo!()
}
