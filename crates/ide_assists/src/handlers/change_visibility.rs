use syntax::ast;

use crate::{utils::vis_offset, AssistContext, AssistId, AssistKind, Assists};

// Assist: change_visibility
//
// Adds or changes existing visibility specifier.
//
// ```
// $0fn frobnicate() {}
// ```
// ->
// ```
// pub(crate) fn frobnicate() {}
// ```
pub(crate) fn change_visibility(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn add_vis(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn change_vis(acc: &mut Assists, vis: ast::Visibility) -> Option<()> {
    todo!()
}
