use common::*;

use hir::{db::HirDatabase, HasSource, HasVisibility, PathResolution};
use ide_db::file_db::FileID;
use syntax::ast;

use crate::{utils::vis_offset, AssistContext, AssistId, AssistKind, Assists};

// FIXME: this really should be a fix for diagnostic, rather than an assist.

// Assist: fix_visibility
//
// Makes inaccessible item public.
//
// ```
// mod m {
//     fn frobnicate() {}
// }
// fn main() {
//     m::frobnicate$0() {}
// }
// ```
// ->
// ```
// mod m {
//     $0pub(crate) fn frobnicate() {}
// }
// fn main() {
//     m::frobnicate() {}
// }
// ```
pub(crate) fn fix_visibility(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    add_vis_to_referenced_module_def(acc, ctx)
        .or_else(|| add_vis_to_referenced_record_field(acc, ctx))
}

fn add_vis_to_referenced_module_def(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn add_vis_to_referenced_record_field(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}
