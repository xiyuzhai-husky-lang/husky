use hir::HirDisplay;
use ide_db::helpers::node_ext::walk_ty;
use syntax::ast::{self, LetStmt, Param};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: add_explicit_type
//
// Specify type for a let binding.
//
// ```
// fn main() {
//     let x$0 = 92;
// }
// ```
// ->
// ```
// fn main() {
//     let x: i32 = 92;
// }
// ```
pub(crate) fn add_explicit_type(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}
